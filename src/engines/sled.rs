use super::KvsEngine;
use crate::error::{KvsError, Result};
use std::path::PathBuf;

pub struct SledKvsEngine {
    pub directory_path: PathBuf,
    pub sled_db: sled::Db,
}

impl SledKvsEngine {
    pub fn open(name: &str) -> Result<sled::Db> {
        sled::open(name).map_err(|err| err.into())
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let _result = self.sled_db.insert(key.as_bytes(), value.as_bytes());

        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let ivec_value = self.sled_db.get(key.as_bytes())?; //TODO! Better error handling for option

        if ivec_value.is_none() {
            return Ok(Some("Key not found".to_string()));
        };

        //TODO! Is there a better way to convert Ivecs into Strings?
        let vec_bytes: Vec<u8> = ivec_value.unwrap().iter().map(|byte| *byte).collect();

        let string = String::from_utf8_lossy(&vec_bytes);

        Ok(Some(string.to_string()))
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let result = self.sled_db.remove(key.as_bytes())?;

        if result.is_none() {
            return Err(KvsError::Store("Key not found".to_owned()));
        }

        Ok(())
    }
}
