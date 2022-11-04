use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

pub struct SeleneSocket {
    pub socket: TcpListener,
}

impl SeleneSocket {
    pub async fn handle_client_connection(listener: &mut TcpListener) -> (TcpStream, SocketAddr) {
        let client = listener.accept().await.unwrap();
        client.0.set_nodelay(true).unwrap();
        return client;
    }
}
