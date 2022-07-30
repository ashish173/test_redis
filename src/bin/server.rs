use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::time::error::Error;

use std::fmt::{self, Formatter};
// use std::fmt::Error;
use std::io::Cursor;
use std::slice::ChunksExact;
use std::{io as Test, str};
use tokio::io::Interest;
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

use bytes::{BufMut, BytesMut};

pub async fn init_connection() {

    // TCPListener
}

use core::fmt::Debug;
pub enum Command {
    Get,
    Set,
    Invalid,
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Command::Get => write!(f, "Get"),
            Command::Set => write!(f, "Set"),
            Command::Invalid => write!(f, "Invalid"),
        }
    }
}

struct Get {
    key: String,
}

impl Get {
    pub fn apply() -> Result<&'static str, Error> {
        let result = "success response";
        Ok(result)
    }
}

struct Set {
    key: String,
    value: String,
}

impl Set {
    pub fn apply(self,) -> Result<&'static str, Error> {
        let result = "success response";
        Ok(result)
    }
}
fn get_command(data: &str) -> Command {
    match data {
        "set" => {
            println!("Set command matched");
            Command::Set
            // let set = Set {
            //     key: "sdf".to_string(),
            //     value: "dfdf".to_string(),
            // };
            // fetch_attrs()
            // set.apply();
        }
        "get" => {
            // let get = Get {
            //     key: "sdf".to_string(),
            //     // value: "dfdf".to_string(),
            // };
            println!("Get command matched");
            Command::Get
        }
        _ => Command::Invalid,
    }
}
async fn process_socket(socket: TcpStream) -> io::Result<()> {
    let server_result = socket.ready(Interest::READABLE).await?;

    if server_result.is_readable() {
        let mut stream = BufWriter::new(socket);

        let mut data= BytesMut::with_capacity(4 * 1024);
        let _val = stream.read_buf(&mut data).await?;
        let mut chunked_data= data.chunks_exact(3);
        let command = std::str::from_utf8(chunked_data.next().unwrap()).unwrap();
        println!("command:{}", command);
        let cmd: Command = get_command(command);
        fetch_attrs(cmd, chunked_data, stream).await?;
    }
    Ok(())
}

pub async fn fetch_attrs(cmd: Command, chunked_data: ChunksExact<'_, u8>, mut stream: BufWriter<TcpStream>) -> std::result::Result<(), std::io::Error> {
    // match command, Get, Set -> Enum Command
    println!("in fetch attrs, {:?}", cmd);
    match cmd {
        Command::Get => {
            println!("get");
            // Get::apply()
            Ok(())
        }
        Command::Set => {
            println!("set");
             stream.write_all(b"Rohit").await?;
            stream.flush().await?;
            // println!("Result{:?}", result);

            // Set::apply(self)
            Ok(())
        }
        Command::Invalid => {
            println!("invalid");
            Ok(())
        }
    }
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
