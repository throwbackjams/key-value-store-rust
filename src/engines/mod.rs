use crate::error::Result;
pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;

    fn get(&mut self, key: String) -> Result<Option<String>>;

    fn remove(&mut self, key: String) -> Result<()>;
}

pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;

mod kvs;
mod sled;
