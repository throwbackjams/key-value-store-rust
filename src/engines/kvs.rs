///Primary struct is a KvStore containing a single HashMap
use std::collections::HashMap;
use std::path::PathBuf;
use crate::utils::{ KVS_FILE_NAME };
use std::io::BufWriter;
use std::fs::{ self, File };
use crate::error::{ KvsError, Result };
use serde::{ Deserialize, Serialize };
use super::KvsEngine;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct KvStore {
    pub kv: Arc<Mutex<HashMap<String, usize>>>,
    pub directory_path: PathBuf,
    pub log_pointer: Arc<Mutex<usize>>,
}

impl KvStore {
    ///Create a hashmap
    pub fn new(path: PathBuf) -> KvStore {
        KvStore {
            kv: Arc::new(Mutex::new(HashMap::new())),
            directory_path: path,
            log_pointer: Arc::new(Mutex::new(0)),
        }
    }

    ///Open the KvStore at a given path. Return the KvStore
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        // println!("opening file");

        let directory: PathBuf = path.into();
        fs::create_dir_all(&directory)?;

        let full_path = directory.join(KVS_FILE_NAME);
        // println!("full path: {:?}", full_path);

        let file = get_file(full_path.clone())?;
        // println!("file opened");

        //read the log file into a series of commands
        let deserialized_commands: Vec<Command> = deserialize_commands_from_file(file);

        // println!("Deserialized Commands: {:?} ", deserialized_commands);

        //"replay" commands into the HashMap in memory -> for each command, match against commands and execute
        let mut in_mem_kv = KvStore::new(directory);
        build_log_pointers(&mut in_mem_kv, deserialized_commands.clone());

        // println!("In memory pointer map: {:?}", in_mem_kv.kv);

        //Compaction
        //println!("Old disc before compaction: {:?} ", deserialized_commands);
        let mut new_disc: Vec<Command> = Vec::new();
        perform_compaction(&mut in_mem_kv, deserialized_commands, &mut new_disc);

        //write new Vec<Command> to disc & check that pointer values in memory reflect correct disc pointer
        //println!("New compacted disc: {:?} ", new_disc);
        //println!("New log pointer map: {:?} ", in_mem_kv.kv);
        //println!("New Store pointer value: {:?}", in_mem_kv.log_pointer);

        // println!("Attempting to write");

        //TODO: Is there a more efficient way to write multiple Commands to disc? Seems like opening a new file handle for each write is inefficient. Perhaps write Vec<Command> to file and figure out how to deserialize that?
        let _clean_file = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(full_path.clone())?;

        for command in new_disc.iter() {
            let file = fs::OpenOptions::new()
                .append(true)
                .open(full_path.clone())?;

            // println!("File opened successfully");

            let f = BufWriter::new(file);

            serde_json::to_writer(f, &command)?;
        }

        // println!("Write complete");
        Ok(in_mem_kv)
    }

    ///  Get the file path for the disc log
    fn get_file_path(&self) -> PathBuf {
        self.directory_path.join("log.txt")
    }
}

impl KvsEngine for KvStore {

    ///Set the value of a string key to a string. Return an error if the value is not written successfully.
    fn set(&self, key: String, value: String) -> Result<()> {
        
        let mut locked_kv = self.kv.lock().expect("Set: lock mutex of hashmap failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work
        let mut locked_log_pointer = self.log_pointer.lock().expect("Set: lock mutex of log pointer failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work
        
        locked_kv.insert(key.clone(), *locked_log_pointer);

        let command = Command::Set { key, value };

        let full_path = self.get_file_path();
        // println!("set full path: {:?}", full_path);

        // let file = File::open(full_path)?;

        let file = get_file(full_path)?;

        serde_json::to_writer(file, &command)?;

        // println!("Set write complete");

        *locked_log_pointer += 1;

        Ok(())
    }

    ///Remove a given key. Return an error if the key does not exist or is not removed successfully.
    fn remove(&self, key: String) -> Result<()> {
        
        let mut locked_kv = self.kv.lock().expect("Set: lock mutex of hashmap failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work
        let mut locked_log_pointer = self.log_pointer.lock().expect("Set: lock mutex of log pointer failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work
        
        let result = locked_kv.remove(&key);

        // println!("Remove result: {:?}", result.clone());

        if result.is_none() {
            return Err(KvsError::Store("Key not found".to_owned()));
        }

        // println!("Writing remove to disk");

        let command = Command::Rm { key };

        let full_path = self.get_file_path();
        // println!("set remove full path: {:?}", full_path);

        // let file = File::open(full_path)?;

        let file = get_file(full_path)?;

        serde_json::to_writer(file, &command)?;
        // println!("Remove write complete");

        *locked_log_pointer += 1;

        Ok(())
    }

    ///Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    fn get(&self, key: String) -> Result<Option<String>> {

        let locked_kv = self.kv.lock().expect("Set: lock mutex of hashmap failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work

        if locked_kv.get(&key).cloned().is_none() {
            return Ok(None);
        }

        let log_pointer = locked_kv.get(&key).unwrap();

        let full_path = self.get_file_path();
        // println!("set remove full path: {:?}", full_path);

        // println!("Opening file on disk as part of GET");
        // println!("full path of GET: {:?}", full_path);

        let file = get_file(full_path)?;

        //TODO! Instead of deserializing every command on get (very inefficient), use seek and command length to jump right to the right spot
        let deserialized_commands: Vec<Command> = deserialize_commands_from_file(file);

        // println!("Deserialized Commands from get: {:?}", deserialized_commands);

        // println!("key: {:?}, pointer value: {:?}", &key, log_pointer);
        // println!("Store pointer value: {:?}", self.log_pointer);

        let command_on_disc = deserialized_commands
            .get(*log_pointer)
            .expect("Log pointer invalid"); //TODO: Should handle this potential error better

        // println!("Deserialized Command found through log pointer: {:?}", command_on_disc);

        if let Command::Set { key: _, value } = command_on_disc {
            Ok(Some(value.to_owned()))
        } else {
            Err(KvsError::Store(
                "Unable to find key through the log pointer".to_owned(),
            ))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Set { key: String, value: String },
    Rm { key: String },
}

///   Open a file given file path
fn get_file(full_path: PathBuf) -> Result<File> {
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(full_path)
        .map_err(|err| err.into())
}

///   Deserialize commands from reader
fn deserialize_commands_from_file(file: File) -> Vec<Command> {
    serde_json::Deserializer::from_reader(file)
        .into_iter::<Command>()
        .flat_map(|it| it.ok())
        .collect::<_>()
}

///Build log pointers for active data in memory
fn build_log_pointers(in_mem_kv: &mut KvStore, deserialized_commands: Vec<Command>) {
    
    let mut locked_kv = in_mem_kv.kv.lock().expect("Set: lock mutex of hashmap failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work
    let mut locked_log_pointer = in_mem_kv.log_pointer.lock().expect("Set: lock mutex of log pointer failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work
                
    
    for command in deserialized_commands.iter() {
        match command {
            Command::Set { key, value: _ } => {
                locked_kv.insert(key.clone(), *locked_log_pointer);
                *locked_log_pointer += 1;
            }
            Command::Rm { key } => {
                locked_kv.remove(key);
                *locked_log_pointer += 1;
            }
        };
    }
}

///Perform compaction given a KvStore
fn perform_compaction(
    in_mem_kv: &mut KvStore,
    deserialized_commands: Vec<Command>,
    new_disc: &mut Vec<Command>,
) {
    //track number of removals
    let mut removals: usize = 0;

    //For (i, command) in deserialized_commands.enumerate()
    //look up command.key in memory hashmap
    //if exits and positon i is equal to hashmap pointer value, then copy to new vec<Command> and set in memory pointer value as (current value minus the removals so far)
    //else increment removal counter by one
    //(Note: if does not exist or exists but position i is less than the hashmap pointer value, then disregard for removal)
    let mut locked_kv = in_mem_kv.kv.lock().expect("Set: lock mutex of hashmap failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work
    let mut locked_log_pointer = in_mem_kv.log_pointer.lock().expect("Set: lock mutex of log pointer failed"); //TODO! Better error handling for PoisonError. A simple From<PoisonError<T>> to KvsError doesn't work

    for (i, command) in deserialized_commands.iter().enumerate() {
        
        match command {
            Command::Rm { key: _ } => continue,
            Command::Set { key, value: _ } => {
                let pointer = locked_kv.get(key);

                if pointer != None && *pointer.unwrap() == i {
                    new_disc.push(command.to_owned());
                    locked_kv.insert(key.to_string(), i - removals);
                } else {
                    removals += 1;
                }
            }
        };
    }
    //reset pointer value
    *locked_log_pointer = new_disc.len();
}
