use std::collections::hash_map::Values;

use bytes::Bytes;

use crate::Db;
use crate::Set;
pub struct Handler {
    db: Db,
}
impl Handler {
    pub fn new(db: Db) -> Handler {
        Handler {
            db: db,
        }
    }
    pub fn write (&mut self, key: &str, value: &'static str){
        let rohit = &self.db.entries.insert(key.to_string(),  Bytes::from(value));
        let result = &self.db.entries.get( &key.to_string());
        println!("result======={:?}",result.unwrap());
    }
    pub async fn run(&mut self){

    }
}
