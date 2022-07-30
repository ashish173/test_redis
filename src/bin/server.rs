use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};

use std::io::Cursor;
use std::{io as Test, str};
use tokio::io::Interest;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

use bytes::{BufMut, BytesMut};

pub async fn init_connection() {

    // TCPListener
}

struct Server {}

async fn parse_data(data: BytesMut) -> io::Result<()> {
    Ok(())
}
async fn process_socket(socket: TcpStream) -> io::Result<()> {
    let server_result = socket.ready(Interest::READABLE).await?;

    if server_result.is_readable() {
        let mut stream = BufWriter::new(socket);

        let mut data = BytesMut::with_capacity(4 * 1024);
        let _val = stream.read_buf(&mut data).await?;
        let mut chunked_data = data.chunks_exact(3);
        let command = std::str::from_utf8(chunked_data.next().unwrap()).unwrap();
        println!("command:{}", command);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("in server");

    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        println!("connection accepted server");
        process_socket(socket).await?;
    }
}
