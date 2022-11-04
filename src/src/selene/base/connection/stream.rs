use tokio::{io::AsyncWriteExt, net::TcpStream};

use super::packets::{handshake::Handshake, utils::write_int_to_stream};

pub struct SeleneStream {}

impl SeleneStream {
    pub async fn send_data(address: &str, port: &str, handshake: Handshake) -> TcpStream {
        let mut outgoing_stream =
            TcpStream::connect(format!("{address}:{port}", address = address, port = port))
                .await
                .unwrap();

        outgoing_stream.set_nodelay(true).unwrap();
        write_int_to_stream(&mut outgoing_stream, handshake.size).await;
        outgoing_stream.write_all(&handshake.body).await.unwrap();

        return outgoing_stream;
    }

    pub async fn setup(address: &str, port: &str) -> TcpStream {
        return TcpStream::connect(format!("{address}:{port}", address = address, port = port))
            .await
            .unwrap();
    }
}
