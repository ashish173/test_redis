use bytes::{buf, BytesMut, Bytes};
use std::{io, thread, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Interest};
use tokio::net::TcpStream;
use clap::{Parser, Subcommand};

// pub fn init_connection() {
//     println!("in client");
// }

// struct Client {}
#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        #[clap(parse(from_str = bytes_from_str))]
        value: Bytes,
    },
}
fn bytes_from_str(src: &str) -> Bytes {
    Bytes::from(src.to_string())
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let args = Cli::parse();
    // init_connection()

    // let mut data= BytesMut::with_capacity(4 * 1024);
    let mut data = BytesMut::with_capacity(4 * 1024);

    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

    // stream.writable().await?;
    stream.write_all(b"set").await?;
    // println!("Result_new{:?}", stream);
    // thread::sleep(time::Duration::from_millis(1000));
    let server_result = stream.ready(Interest::READABLE).await?;

    if server_result.is_readable() {
        stream.read_buf(&mut data).await?;
        {
            if let Ok(string) = std::str::from_utf8(&mut data) {
                println!("\"{}\"", string);
            } else {
                println!("{:?}", data);
            }
        };
    }
    Ok(())
}
