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
use std::process;

///Primary struct is a KvStore containing a single HashMap
pub struct KvStore {
    pub kv: HashMap<String, u64>, //Change to store log pointer
    pub directory_path: PathBuf,
    pub log_pointer: u64,
}

///Result wrapper to consolidate program errors
pub type Result<T> = std::result::Result<T, KvsError>;

///Custom errors for the program
#[derive(Debug)]
pub enum KvsError{
    Io(io::Error),
    Serde(serde_json::Error),
    Store(String),
}

impl fmt::Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KvsError::Io(err) => write!(f, "IO error: {}", err),
            KvsError::Serde(err) => write!(f, "Serde error: {}", err),
            KvsError::Store(err) => write!(f, "{}", err),
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
            directory_path: path,
            log_pointer: 0,
        }
    }

    ///Set the value of a string key to a string. Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        
        self.kv.insert(key.clone(), self.log_pointer);

        let command = Command::Set{ key, value };

        let directory: &PathBuf = &self.directory_path;

        let full_path = directory.join("log.txt");
        // println!("set full path: {:?}", full_path);

        // let file = File::open(full_path)?;


        let file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .read(true)
                        .open(full_path)?;  //&self.directory_path.join("log.txt")
        serde_json::to_writer(file, &command)?;

        // println!("Set write complete");

        self.log_pointer += 1;

        Ok(())
    }

    ///Remove a given key. Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()>{
        let result = self.kv.remove(&key);

        // println!("Remove result: {:?}", result.clone());
        
        if let None = result {
            return Err(KvsError::Store("Key not found".to_owned()))
        }

        // println!("Writing remove to disk");

        let command = Command::Rm { key };

        let directory: &PathBuf = &self.directory_path;

        let full_path = directory.join("log.txt");
        // println!("set remove full path: {:?}", full_path);

        // let file = File::open(full_path)?;

        let file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(full_path)?;

        serde_json::to_writer(file, &command)?;
        // println!("Remove write complete");

        self.log_pointer += 1;

        Ok(())
    }

        ///Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        
        if let None =  self.kv.get(&key).cloned() {
            return Ok(None)
        }

        let log_pointer = self.kv.get(&key).unwrap();

        let directory: &PathBuf = &self.directory_path;

        let full_path = directory.join("log.txt");

        // println!("Opening file on disk as part of GET");
        // println!("full path of GET: {:?}", full_path);

        let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(full_path)?;

        let deserialized_commands: Vec<Command> = serde_json::Deserializer::from_reader(file)
                                                            .into_iter::<Command>()
                                                            .filter_map(|it| it.ok())
                                                            .collect::<_>();

        let command_on_disc = deserialized_commands.iter().nth(*log_pointer as usize).unwrap(); //TODO: Need to handle this potential error better
        
        println!("Deserialized Command found through log pointer: {:?}", command_on_disc);

        if let Command::Set{ key , value } = *command_on_disc {
            return Ok(Some(value))
        } else {
            return Err(KvsError::Store("Unable to find key through the log pointer".to_owned()))
        }

        // println!("Deserialized Commands within GET: {:?} ", deserialized_commands);

        //"replay" commands into the HashMap in memory -> for each command, match against commands and execute

        // let mut in_mem_kv =  HashMap::new();

        // for command in deserialized_commands.iter() {
        //     match command {
        //         Command::Set { key, value } => in_mem_kv.insert(key.clone(), value.clone()),
        //         Command::Rm { key } => in_mem_kv.remove(key),
        //         _ => {
        //             continue
        //         },
        //     };

        // };

        // let result = in_mem_kv.get(&key).cloned();

        // println!("Result is: {:?}", result);

        // if let None = result {
        //     return Err(KvsError::Store("Key not found".to_owned()))
        // }
        
    }

    ///Open the KvStore at a given path. Return the KvStore
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        // println!("called KvStore::open()");
        
        //argument: path: impl Into<PathBuf>

        // let mut file = File::open(path)?;
        // let mut buf = BufReader::new(file)?;

        // // let serialized_string = String::new();

        // // buf.read_to_string(&serialized_string)?;

        // // println!("Serialized string: {:?}", serialized_string);

        // // let deserialized_kv:  = serde_json::Deserializer::from_str(&serialized_string);

        //open the log file
        // println!("opening file");

        let directory: PathBuf = path.into();
        fs::create_dir_all(&directory)?;

        let full_path = directory.join("log.txt");
        // println!("full path: {:?}", full_path);

        // let file = File::open(full_path)?;

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(full_path)?;
        
        // println!("file opened");

        // let mut string = String::new();

        // file.clone().read_to_string(&mut string)?;

        // println!("File contents in string: {:?}", string);

        //read the log file into a series of commands
        let deserialized_commands: Vec<Command> = serde_json::Deserializer::from_reader(file)
                                                            .into_iter::<Command>()
                                                            .filter_map(|it| it.ok())
                                                            .collect::<_>();

        // println!("Deserialized Commands: {:?} ", deserialized_commands);

        //"replay" commands into the HashMap in memory -> for each command, match against commands and execute

        let mut in_mem_kv =  KvStore::new(directory);

        for command in deserialized_commands.iter() {
            match command {
                Command::Set { key, value } => in_mem_kv.kv.insert(key.clone(), value.clone()),
                Command::Rm { key } => in_mem_kv.kv.remove(key),
                _ => {
                    continue
                },
            };

        };

        // println!("In memory hashmap: {:?}", in_mem_kv.kv);

        Ok(in_mem_kv)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Set{ key: String, value: String},
    Get{ key: String },
    Rm{ key: String },
}
