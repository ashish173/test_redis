use bytes::BytesMut;
// use db::Db;
use std::fmt::{self, Formatter};
use std::str;
use test_redis::{buffer_to_array, Db};
use tokio::io::Interest;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream};

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

fn get_command(data: &str) -> Command {
    match data {
        "set" => Command::Set,
        "get" => Command::Get,
        _ => Command::Invalid,
    }
}
async fn process_socket(socket: TcpStream, handler: &mut Db) -> io::Result<()> {
    let server_result = socket.ready(Interest::READABLE).await?;

    if server_result.is_readable() {
        let mut stream = BufWriter::new(socket);

        let mut data = BytesMut::with_capacity(4 * 1024);
        let _val = stream.read_buf(&mut data).await?;
        fetch_attrs(stream, handler, &mut data).await?;
    }
    Ok(())
}

pub async fn fetch_attrs(
    mut stream: BufWriter<TcpStream>,
    handler: &mut Db,
    data: &mut BytesMut,
) -> std::result::Result<(), std::io::Error> {
    let cla_attrs = buffer_to_array(data);

    let command = get_command(&cla_attrs[0]);
    match command {
        Command::Get => {
            let result = handler.read(&cla_attrs);
            match result {
                Ok(result) => {
                    stream.write_all(&result).await?;
                }
                Err(_err) => {
                    stream.write_all(b"").await?;
                }
            }

            stream.flush().await?;
            Ok(())
        }
        Command::Set => {
            let resp = handler.write(&cla_attrs);
            match resp {
                Ok(result) => {
                    stream.write_all(&result.as_bytes()).await?;
                }
                Err(_err) => {
                    stream.write_all(b"").await?;
                }
            }

            stream.flush().await?;
            Ok(())
        }
        Command::Invalid => Ok(()),
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let db = Db::new();
    loop {
        // TODO move this to seperate Listner.listen method; It should call socket accept.
        let (socket, _) = listener.accept().await?;
        let mut new_db = db.clone();
        process_socket(socket, &mut new_db).await?;
    }
}
