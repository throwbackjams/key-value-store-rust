// #![deny(missing_docs)]
//!An implementation of a key value store in Rust
use std::collections::HashMap;
use std::path::PathBuf;
use std::io;
use std::fmt;
use std::error;

///Primary struct is a KvStore containing a single HashMap
pub struct KvStore {
    kv: HashMap<String, String>,
}

///Result wrapper to consolidate program errors
pub type Result<T> = std::result::Result<T, KvsError>;

///Custom errors for the program
#[derive(Debug)]
pub enum KvsError{
    Io(io::Error),
}

impl fmt::Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KvsError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl KvStore {
    ///Create a hashmap
    pub fn new() -> KvStore {
        KvStore { kv: HashMap::new() }
    }

    ///Set the value of a string key to a string. Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.kv.insert(key, value);
        todo!()
    }

    ///Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.kv.get(&key).cloned();
        todo!()
        
    }

    ///Remove a given key. Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()>{
        self.kv.remove(&key);
        todo!()
    }

    ///Open the KvStore at a given path. Return the KvStore
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        todo!()
    }
}
