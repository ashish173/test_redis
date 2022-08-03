use bytes::BytesMut;
use std::fmt::{self, Formatter};
use std::str;
use test_redis::handler::Handler;
use test_redis::{buffer_to_array, Db};
use tokio::io::Interest;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::error::Error;

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

// TODO move this to a separate file
impl Get {
    // Unused so far
    // TODO move the actual data reading to this Get#apply implementation
    pub fn apply() -> Result<&'static str, Error> {
        let result = "success response";
        Ok(result)
    }
}
fn get_command(data: &str) -> Command {
    match data {
        "set" => Command::Set,
        "get" => Command::Get,
        _ => Command::Invalid,
    }
}
async fn process_socket(socket: TcpStream, handler: &mut Handler) -> io::Result<()> {
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
    handler: &mut Handler,
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
                Err(err) => {
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
                Err(err) => {
                    stream.write_all(b"").await?;
                }
            }

            stream.flush().await?;
            Ok(())
        }
        Command::Invalid => Ok(()),
    }
}
struct Listener {
    listener: TcpListener,
    db: Db,
}
impl Listener {
    pub fn new(listener: TcpListener) -> Listener {
        Listener {
            listener: listener,
            db: Db::new(),
        }
    }

    pub async fn accept(&self) -> std::result::Result<TcpStream, std::io::Error> {
        match self.listener.accept().await {
            Ok((socket, _)) => return Ok(socket),
            Err(err) => {
                return Err(err.into());
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let listener = Listener::new(listener);

    loop {
        // TODO move this to seperate Listner.listen method; It should call socket accept.
        let socket = listener.accept().await?;
        let new_db = listener.db.clone();
        let mut handler = Handler::new(new_db);
        process_socket(socket, &mut handler).await?;
    }
}
