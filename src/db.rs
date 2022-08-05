use bytes::Bytes;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Db {
    pub entries: Rc<RefCell<HashMap<String, Bytes>>>,
}
impl Db {
    pub fn new() -> Db {
        Db {
            entries: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];

        // we need to clone the referenced value since Bytes::from() function expects a 'static lifetime
        // variable but `value` has unknown lifetime in this function context
        let val = value.clone();

        let p = &self
            .entries
            .borrow_mut()
            .insert(String::from(key), Bytes::from(val));

        match p {
            Some(_p) => Ok("r Ok"),
            None => Ok("Ok"),
        }
    }

    /// Reads data from the database
    pub fn read(&mut self, arr: &[String]) -> Result<Bytes, &str> {
        let key = &arr[1];
        let db_copy = &self.entries.try_borrow().unwrap();
        let db_clone = db_copy.clone();
        let query_result = db_clone.get(key);

        if let Some(value) = query_result {
            // this cloning is needed because of the below error
            // Error: cannot return value referencing temporary value
            // returns a value referencing data owned by the current function
            // sol: we create a clone of the temporary value and then return it.
            let cloned_value = value.clone();
            return Ok(cloned_value);
        } else {
            return Err("no such key found");
        }
    }
}
