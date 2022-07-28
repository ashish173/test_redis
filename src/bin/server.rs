use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};

use std::{io as Test, str};
use tokio::io::Interest;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

use bytes::{BufMut, BytesMut};

pub async fn init_connection() {

    // TCPListener
}

struct Server {}

async fn parse_data(data: &str, stream: &mut BufWriter<TcpStream>) -> io::Result<()> {
    // get command name (set, get)
    println!("In Parse Data");
    println!("length {}", data.len());
    let mut buffer = BytesMut::with_capacity(64);
    // let in_bytes = data.as_bytes();
    // let a = "hdhd".to_string();
    // let b = &b"df"[..];

    // buffer.put(in_bytes);
    // // buffer.put_slice(in_bytes);

    println!("Stream {:?}", stream);

    let count = stream.read_buf(&mut buffer).await?;

    println!(" Buffer {:?}", buffer);

    // let split: Vec<&str> = data.trim().split(' ').collect();
    // println!("split {:?}", split);

    // based on command call the command::apply method
    Ok(())
}

async fn process_socket(socket: TcpStream) -> io::Result<()> {
    // println!("socket {:?}", socket);
    let server_result = socket.ready(Interest::READABLE).await?;
    println!("{:?}", server_result);

    if server_result.is_readable() {
        println!("value is readable");
        let mut data = vec![0; 1024];
        let val = socket.try_read(&mut data);
        // println!("result:{:?}", val);
        // println!("result:{:?}", data);

        let mut stream = BufWriter::new(socket);

        let str_data = std::str::from_utf8(&data);
        match str_data {
            Ok(v) => {
                print!("matched value {}", v);
                parse_data(v, &mut stream).await?;
            }
            Err(v) => print!("{}", v),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::create("foo.txt").await?;
    {
        let mut writer = BufWriter::new(f);

        // Write a byte to the buffer.
        writer.write(&[42u8]).await?;

        // Flush the buffer before it goes out of scope.
        writer.flush().await?;
    } // Unless flushed or shut down, the contents of the buffer is discarded on drop.

    println!("in server");
    // Listen on socket
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    // let u_listener = UnixListener::bind("127.0.0.1:8080").unwrap();

    loop {
        let (socket, _) = listener.accept().await?;
        println!("connection accepted server");
        // let (socket_u, _) = u_listener.accept().await?;
        process_socket(socket).await;
        // process_socket(socket_u).await;
    }
}
