use bytes::{Buf, Bytes, BytesMut};
use std::{collections::HashMap, rc::Rc, cell::RefCell};
use tokio::time::error::Error;
pub mod client;
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
    pub fn insert(key: String, value: String){
        println!("Key:{}====value=={}", key, value);
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

pub fn buffer_to_array(buf: &mut BytesMut) -> Vec<String> {
    let mut vec = vec![];
    let length = buf.len();
    let mut word = "".to_string();

    for i in 0..length {
        match buf.get_u8() {
            b' ' => {
                // insert the word into vector
                println!("in space {}", word);

                vec.push(word);
                word = "".to_string();
            }
            (test) => {
                // increase the word
                println!("other than space: {}", test);
                word.push(test as char);
                let new = word.clone();
                if (i == length - 1) {
                    vec.push(new);
                }
            }
        }
    }

    // vec.push("set".to_string());
    // vec.push("foo".to_string());
    // vec.push("bar".to_string());
    println!("final vector {:?}", vec);
    vec
}
