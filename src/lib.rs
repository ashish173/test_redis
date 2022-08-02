use std::collections::HashMap;
use bytes::Bytes;
use tokio::time::error::Error;
pub mod client;
pub mod handler;
// pub server:: mod db;
#[derive(Clone, Debug)]
pub struct Db{
    entries: HashMap<String, Bytes>
}
impl Db {
   pub fn new() -> Db{
    Db{entries: HashMap::new()}
   }
}

pub struct Set {
    key: String,
    value: String,
}

impl Set {
    pub fn apply(self) -> Result<&'static str, Error> {
        let result = "success response";
        Ok(result)
    }
}