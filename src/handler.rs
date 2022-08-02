use std::thread::sleep;
use std::time::Duration;

use std::collections::hash_map::Values;
use std::io::ErrorKind;

use bytes::Bytes;
use tokio::time::error::Error;

use crate::Db;
use crate::Set;
pub struct Handler {
    db: Db,
}
impl Handler {
    pub fn new(db: Db) -> Handler {
        Handler { db: db }
    }
    pub fn write(&mut self, arr: &[String]) {
        let key = &arr[1];
        let value = &arr[2];
        // let val = value.to_string();
        // let new_val = "df";
        println!("value: {:?}", value.clone());
        let val = value.clone();

        &self.db.entries.insert(String::from(key), Bytes::from(val));

        // sleep(Duration::from_millis(4000));
        // Ok(("success".to_string()))
    }
    pub fn read(&mut self, arr: &[String]) {
        let key = &arr[1];

        let result = &self.db.entries.get(&key.to_string());
        println!("{}: {:?}", key, result.unwrap());
    }
}
