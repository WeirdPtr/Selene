use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn read_int_from_stream<T>(stream: &mut T) -> i32
where
    T: AsyncRead + std::marker::Unpin,
{
    let mut num_read: i32 = 0;
    let mut res: i32 = 0;
    loop {
        let read = stream.read_u8().await.unwrap() as i32;
        let value = read & 0b0111_1111;
        res |= value << (7 * num_read);
        num_read += 1;
        if num_read > 5 {
            return -10;
        }
        if (read & 0b1000_0000) == 0 {
            break;
        }
    }
    return res;
}

pub async fn write_int_to_stream<T>(stream: &mut T, mut value: i32)
where
    T: AsyncWrite + std::marker::Unpin,
{
    loop {
        let mut temp: i16 = (value & 0b0111_1111) as i16;
        value >>= 7;
        if value != 0 {
            temp |= 0b1000_0000;
        }
        stream.write_i8(temp as i8).await.unwrap();
        if value == 0 {
            break;
        }
    }
}
pub async fn read_string_from_stream<T>(stream: &mut T) -> String
where
    T: AsyncRead + std::marker::Unpin,
{
    let length = read_int_from_stream(stream).await;
    let mut buf = vec![0u8; length as usize];
    stream.read_exact(&mut buf).await.unwrap();
    return String::from_utf8_lossy(&buf).to_string();
}
