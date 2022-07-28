use std::io;
use tokio::net::TcpStream;

pub fn init_connection() {
    println!("in client");
}

struct Client {}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    // init_connection()
    let stream = TcpStream::connect("127.0.0.1:8081").await?;
    stream.writable().await?;
    stream.try_write(b"Hello");
    Ok(())
}
