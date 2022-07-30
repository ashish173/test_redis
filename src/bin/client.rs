use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub fn init_connection() {
    println!("in client");
}

struct Client {}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    // init_connection()
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    stream.writable().await?;
    stream.write_all(b"set foo").await?;
    Ok(())
}
