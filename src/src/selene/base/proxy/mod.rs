use std::sync::Arc;

use lightlog::{Logger, LoggingType};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::selene::base::connection::packets::handshake::Handshake;

use super::connection::{listener::SeleneSocket, stream::SeleneStream};

pub struct SeleneProxy {
    pub upstream: SeleneSocket,
    pub downstream: TcpStream,
    pub proxy_address: String,
    pub proxy_port: String,
    pub target_address: String,
    pub target_port: String,
    logger: Arc<Logger>,
}

impl SeleneProxy {
    pub async fn initialize(
        proxy_address: &str,
        proxy_port: i16,
        target_address: &str,
        target_port: i16,
        logger: Arc<Logger>,
    ) -> SeleneProxy {
        let socket = SeleneSocket {
            socket: TcpListener::bind(format!(
                "{address}:{port}",
                address = proxy_address.to_owned(),
                port = proxy_port.to_owned()
            ))
            .await
            .unwrap(),
        };

        let listener = SeleneStream::setup(target_address, &target_port.to_string()).await;

        SeleneProxy {
            upstream: socket,
            downstream: listener,
            proxy_address: proxy_address.to_owned(),
            proxy_port: proxy_port.to_string(),
            target_address: target_address.to_owned(),
            target_port: target_port.to_string(),
            logger,
        }
    }

    pub async fn run(self) {
        let instance = Arc::new(Mutex::new(self));

        loop {
            let loop_instance = instance.clone();
            let logger = loop_instance.lock().await.logger.clone();

            let mut stream = SeleneSocket::handle_client_connection(
                &mut loop_instance.lock().await.upstream.socket,
            )
            .await
            .0;

            let handle = tokio::spawn(async move {
                logger.log_message("Connection Transfer requested", LoggingType::Debug);

                let handshake = Handshake::read_from_tcp_stream(&mut stream).await;

                if handshake.is_none() {
                    return;
                }

                let handshake = handshake.unwrap();

                let lock = loop_instance.lock().await;

                let sent_data_stream = SeleneStream::send_data(
                    &lock.target_address.to_owned(),
                    &lock.target_port.to_owned(),
                    handshake,
                )
                .await;

                let (mut client_stream_reader, mut client_stream_writer) = tokio::io::split(stream);
                let (mut target_stream_reader, mut target_stream_writer) =
                    tokio::io::split(sent_data_stream);

                tokio::spawn(async move {
                    let result =
                        tokio::io::copy(&mut client_stream_reader, &mut target_stream_writer).await;

                    if let Some(err) = result.err() {
                        let err_str = format!(
                            "Error occurred in client-server bridge. Server may have disconnected: {error}",
                            error = err
                        );

                        logger.log_origin_message(err_str, LoggingType::Error, None);
                    }
                });
                
                tokio::spawn(async move {
                    let _ =
                        tokio::io::copy(&mut target_stream_reader, &mut client_stream_writer).await;
                });
            });

            handle.await.unwrap();
        }
    }
}
