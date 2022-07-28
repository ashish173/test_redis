use std::{io, str};
use tokio::io::Interest;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

use bytes::{BufMut, BytesMut};

pub async fn init_connection() {

    // TCPListener
}

struct Server {}

fn parse_data(data: &str) {
    // get command name (set, get)
    println!("length {}", data.len());
    let mut buffer = BytesMut::with_capacity(64);
    let in_bytes = data.as_bytes();
    buffer.put(&in_bytes[..]);

    println!("Buffer {:?}", buffer);
    // let split: Vec<&str> = data.trim().split(' ').collect();
    // println!("split {:?}", split);

    // based on command call the command::apply method
}

async fn process_socket(socket: TcpStream) -> io::Result<()> {
    println!("socket {:?}", socket);
    let server_result = socket.ready(Interest::READABLE).await?;
    println!("{:?}", server_result);
    if server_result.is_readable() {
        let mut data = vec![0; 1024];
        let val = socket.try_read(&mut data);
        println!("result:{:?}", val);
        println!("result:{:?}", data);

        let str_data = std::str::from_utf8(&data);
        match str_data {
            Ok(v) => {
                print!("{}", v);
                parse_data(v);
            }
            Err(v) => print!("{}", v),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
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
