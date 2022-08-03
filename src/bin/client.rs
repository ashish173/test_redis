use bytes::{buf, Bytes, BytesMut};
use clap::{Parser, Subcommand};
use std::fmt::Error;
use std::{io, thread, time};
// use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Interest};
use tokio::net::TcpStream;

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

    let mut data = BytesMut::with_capacity(4 * 1024);
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    match args.command {
        Command::Set { key, value } => {
            stream.write_all(b"set").await?;
            stream.write_all(b" ").await?;

            stream.write_all(&key.as_bytes()).await?;
            stream.write_all(b" ").await?;

            stream.write_all(&value).await?;
            let mut buf = BytesMut::with_capacity(1024);
            let _length = stream.read_buf(&mut buf).await?;
            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    if *resp == "Ok".to_string() {
                        println!("\"Ok\"");
                    } else if resp.to_string() == "Ok".to_string() {
                        println!("had to convert first");
                    }
                }
                Err(_err) => {
                    println!("in errr");
                }
            }
        }
        Command::Get { key } => {
            stream.write_all(b"get").await?;
            stream.write_all(b" ").await?;

            stream.write_all(&key.as_bytes()).await?;
            {
                if let Ok(string) = std::str::from_utf8(&mut data) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", data);
                }
            };
            return Ok(());
        }
    }

    Ok(())
}
