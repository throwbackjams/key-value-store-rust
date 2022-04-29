// #![deny(missing_docs)]
//!An implementation of a key value store in Rust
use std::collections::HashMap;
use std::path::PathBuf;
use std::io:: {self, Write, Read, BufReader};
use std::fmt;
use std::error;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use serde_json;

///Primary struct is a KvStore containing a single HashMap
pub struct KvStore {
    pub kv: HashMap<String, String>,
    pub path: PathBuf,
}

///Result wrapper to consolidate program errors
pub type Result<T> = std::result::Result<T, KvsError>;

///Custom errors for the program
#[derive(Debug)]
pub enum KvsError{
    Io(io::Error),
    Serde(serde_json::Error),
}

impl fmt::Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KvsError::Io(err) => write!(f, "IO error: {}", err),
            KvsError::Serde(err) => write!(f, "Serde error: {}", err),
        }
    }
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError{
        KvsError::Serde(err)
    }    
}

impl KvStore {
    ///Create a hashmap
    pub fn new(path: PathBuf) -> KvStore {
        KvStore { 
            kv: HashMap::new(),
            path: path,
        }
    }

    ///Set the value of a string key to a string. Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        
        self.kv.insert(key.clone(), value.clone());

        let command = Command::Set{ key, value };

        let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(&self.path)?;

        let serialized_command = serde_json::to_writer(file, &command)?;

        Ok(())
    }

    ///Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let result = self.kv.get(&key).cloned();
        Ok(result)
        
    }

    ///Remove a given key. Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()>{
        self.kv.remove(&key);
        
        let command = Command::Rm { key };

        let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(&self.path)?;

        let serialized_command = serde_json::to_writer(file, &command)?;

        Ok(())
    }

    ///Open the KvStore at a given path. Return the KvStore
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        println!("called KvStore::open()");
        
        //argument: path: impl Into<PathBuf>

        // let mut file = File::open(path)?;
        // let mut buf = BufReader::new(file)?;

        // // let serialized_string = String::new();

        // // buf.read_to_string(&serialized_string)?;

        // // println!("Serialized string: {:?}", serialized_string);

        // // let deserialized_kv:  = serde_json::Deserializer::from_str(&serialized_string);

        //open the log file
        println!("opening file");
        
        let path_buf: PathBuf = path.into();
        let file = File::open(path_buf.as_path())?;
        
        println!("file opened");

        // let mut string = String::new();

        // file.read_to_string(&mut string)?;

        // println!("File contents in string: {:?}", string);

        //read the log file into a series of commands
        let deserialized_commands: Vec<Command> = serde_json::Deserializer::from_reader(file)
                                                            .into_iter::<Command>()
                                                            .filter_map(|it| it.ok())
                                                            .collect::<_>();

        println!("Deserialized Commands: {:?} ", deserialized_commands);

        //"replay" commands into the HashMap in memory -> for each command, match against commands and execute

        let mut in_mem_kv =  KvStore::new(path_buf);

        for command in deserialized_commands.iter() {
            match command {
                Command::Set { key, value } => in_mem_kv.kv.insert(key.clone(), value.clone()),
                Command::Rm { key } => in_mem_kv.kv.remove(key),
                _ => {
                    println!("ignored");
                    continue
                },
            };

        };

        println!("In memory hashmap: {:?}", in_mem_kv.kv);

        Ok(in_mem_kv)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Set{ key: String, value: String},
    Get{ key: String },
    Rm{ key: String },
}
