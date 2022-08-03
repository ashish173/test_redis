use std::borrow::Borrow;
use std::collections::HashMap;
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
// let target = HashMap::new();
        let ptr = &self.db.entries.borrow_mut().insert(String::from(key), Bytes::from(val));
        // insert(String::from(key), Bytes::from(val));
        // &self.db.entries.;
        // ptr.insert(String::from(key), Bytes::from(val));
        println!("Db entries=={:?}", &self.db.entries);
        let copy = &self.db.entries.borrow_mut();
        let result = copy.get(&key.to_string());

        println!("Result ==={:?}", result.unwrap());
        // sleep(Duration::from_millis(4000));
        // Ok(("success".to_string()))
    }
    pub fn read(&mut self, arr: &[String]) {
        let key = &arr[1];
        // println!("latest entries==={:?}", &self.db.entries);

        println!("Db entries=={:?}", &self.db.entries);
        let copy = &self.db.entries.try_borrow().unwrap();
        let result = copy.get(&key.to_string());
        println!("key: {}: value: {:?}", key, result.unwrap());
        
        // Ok(result)
    }
}
