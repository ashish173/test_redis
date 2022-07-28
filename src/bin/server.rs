use std::{io, str};
use tokio::io::Interest;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

pub async fn init_connection() {

    // TCPListener
}

struct Server {}

async fn process_socket(socket: TcpStream) -> io::Result<()> {
    println!("socket {:?}", socket);
    let server_result = socket.ready(Interest::READABLE).await?;
    println!("{:?}", server_result);
    if server_result.is_readable() {
        let mut data = vec![0; 1024];
        let val = socket.try_read(&mut data);
        println!("result:{:?}", val);
        // println!("result:{:?}", std::str::from_utf8(&data));

        let str_data = std::str::from_utf8(&data);
        match str_data {
            Ok(v) => print!("{}", v),
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
