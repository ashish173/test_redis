use bytes::{Buf, Bytes, BytesMut};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tokio::time::error::Error;
pub mod handler;
// pub server:: mod db;
#[derive(Clone, Debug)]
pub struct Db {
    entries: Rc<RefCell<HashMap<String, Bytes>>>,
}
impl Db {
    pub fn new() -> Db {
        Db {
            entries: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

pub struct Set {
    key: String,
    value: String,
}

// TODO move this to a separate file
impl Set {
    // TODO move the actual data reading to this Set#apply implementation
    pub fn apply(self) -> Result<&'static str, Error> {
        let result = "success response";
        Ok(result)
    }
}

pub fn buffer_to_array(buf: &mut BytesMut) -> Vec<String> {
    let mut vec = vec![];
    let length = buf.len();
    let mut word = "".to_string();

    for i in 0..length {
        match buf.get_u8() {
            b' ' => {
                vec.push(word);
                word = "".to_string();
            }
            test => {
                // increase the word
                word.push(test as char);
                let new = word.clone();
                if i == length - 1 {
                    vec.push(new);
                }
            }
        }
    }

    vec
}
