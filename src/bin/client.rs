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

    // let mut data= BytesMut::with_capacity(4 * 1024);
    let mut data = BytesMut::with_capacity(4 * 1024);
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();

    // println!("in line 43");

    println!("{:?}", args.command);

    match args.command {
        Command::Set { key, value } => {
            // client.set(&key, value).await.unwrap();
            stream.write_all(b"set").await?;
            stream.write_all(b" ").await?;

            stream.write_all(&key.as_bytes()).await?;
            stream.write_all(b" ").await?;

            stream.write_all(&value).await?;
            // println!("i am inside set");
            // println!("{:?}", key);
            // println!("{:?}", value);
            println!("waiting to receive successs notification data");
            let mut buf = BytesMut::with_capacity(1024);
            let length = stream.read_buf(&mut buf).await?;
            println!("buffer {:?}", buf);
            // return Ok(());
            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    println!("okay response");
                    println!("response: {}", resp);
                    if (*resp == "Ok".to_string()) {
                        println!("okay matched");
                    } else if (resp.to_string() == "Ok".to_string()) {
                        println!("had to convert first");
                    }
                    // return Ok((string));
                } // Error
                Err(err) => {
                    println!("in errr");
                    // return Err(err.into());
                }
            }
            // println!("OK");
        }
        Command::Get { key } => {
            //     println!("i am inside get");
            //     println!("{:?}", key);
            //     let server_result = stream.ready(Interest::READABLE).await.unwrap();
            //     if server_result.is_readable() {
            //         stream.read_buf(&mut data).await?;
            //         {
            //             if let Ok(string) = std::str::from_utf8(&mut data) {
            //                 println!("\"{}\"", string);
            //             } else {
            //                 println!("{:?}", data);
            //             }
            //         };
            //     }
            // return Ok(());
        }
    }

    Ok(())
}
