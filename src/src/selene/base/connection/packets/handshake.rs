use std::io::Cursor;
use tokio::{io::AsyncReadExt, net::TcpStream};

use super::utils::{read_int_from_stream, read_string_from_stream};

#[derive(Debug)]
pub struct Handshake {
    pub body: Vec<u8>,
    pub size: i32,
    pub host: String,
    pub port: u16,
}

impl Handshake {
    pub async fn read_from_tcp_stream(stream: &mut TcpStream) -> Option<Self> {
        let size = read_int_from_stream(stream).await;
        let mut body = vec![0u8; size as usize];
        let _ = stream.read_exact(&mut body).await;

        let mut body = Cursor::new(body);

        let id = read_int_from_stream(&mut body).await;
        if id != 0 {
            return None;
        }

        read_int_from_stream(&mut body).await;
        let host = read_string_from_stream(&mut body).await;
        let port = body.read_u16().await;

        if port.is_err() {
            return None;
        }

        let port = port.unwrap();

        return Some(Self {
            body: body.into_inner(),
            size,
            host,
            port,
        });
    }
}
