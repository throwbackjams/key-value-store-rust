// #![deny(missing_docs)]
//!An implementation of a key value store in Rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write, BufWriter};
use std::path::PathBuf;
use std::net::{self, IpAddr, TcpListener, TcpStream, Ipv4Addr, Ipv6Addr, AddrParseError};
use std::str::FromStr;

use tracing::{info, debug};
use tracing_subscriber;

const GET: &[u8] = b"GET";
const SET: &[u8] = b"SET";
const RM: &[u8] = b"RM";
const OK_RESPONSE: &[u8] = b"+OK\r\n";

pub struct KvsClient{}

impl KvsClient {

    //TODO! IP address parsing
    //TODO! Send operation to KvsServer at a certain IP
    //TODO! Receive response and proprogate to CLI (success or error)
    pub fn connect_and_send_request(ip_string: String, message: String) -> Result<String> {
        // let _verified_ip_address = parse_ip(&ip_string)?;

        let mut stream = TcpStream::connect(ip_string)?;

        stream.write(message.as_bytes())?;

        let mut buffer = [0; 1024];

        stream.read(&mut buffer)?;

        let string_response = String::from_utf8(buffer[..].to_vec())?;

        Ok(string_response)
    }

}

pub struct KvsServer{}

impl KvsServer{
    
    pub fn listen_and_serve_requests(ip_string: String) -> Result<()>{

        let path = PathBuf::from("");

        let listener = TcpListener::bind(ip_string)?;

        for stream in listener.incoming() {
            
            let kv_store = KvStore::open(&path)?; //Q: What happens if two simultaneous connections occur? Race?
            
            let unwrapped_stream = stream?;

            KvsServer::handle_request(unwrapped_stream, kv_store)?
        }

        Ok(())
    }

    //TODO! Perform operation by calling KvsEngine
    fn handle_request(mut stream: TcpStream, mut kv_store: KvStore ) -> Result<()> {

        let subscriber = tracing_subscriber::FmtSubscriber::new();

        info!("Connection initiated");
        
        let mut buffer = [0; 1024];

        stream.read(&mut buffer)?;

        //Split arguments by space
        let arguments:Vec<&[u8]> = buffer.split(|byte| &[*byte] == b"\n").collect();

        //TODO! translate bytes in the buffer to commands
        match arguments.get(0) {
            Some(&GET) => {
                info!("Processing GET Request");
                //decode key
                let key_bytes = arguments
                            .get(1)
                            .ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                
                if let Err(error) = key_bytes {
                    return Err(error)
                }

                let key = String::from_utf8(key_bytes.unwrap().to_vec())?; //NOTE! Is there a better way to handle this?

                //Handle get request (send response back)
                let result = kv_store.get(key)?;

                info!("Get result: {:?}", result);

                //NOTE! If the result is not Ok(value), then error should propogate to kvs-server and the below should not execute right?
                //Send result back (encapsulate in function?)
                let response = format!("+{}\r\n", result.unwrap_or("None".to_string()));

                stream.write(response.as_bytes())?;
                stream.flush()?;
                
            },
            Some(&SET) => {
                info!("Processing SET Request");
                //decode key and value
                let key_bytes = arguments.get(1).ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                let value_bytes = arguments.get(2).ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                
                //Handle set request (send success reponse)

                if let Err(error) = key_bytes {
                    return Err(error)
                }

                if let Err(error) = value_bytes {
                    return Err(error)
                }

                let key = String::from_utf8(key_bytes.unwrap().to_vec())?;
                let value = String::from_utf8(value_bytes.unwrap().to_vec())?;

                let _result = kv_store.set(key, value)?;

                //NOTE! If the result is not Ok(value), then error should propogate to kvs-server and the below should not execute right?
                //Send result back (encapsulate in function?)
                let response = OK_RESPONSE;

                stream.write(response)?;
                stream.flush()?;



            },
            Some(&RM) => {
                info!("Processing Remove Request");
                //decode key
                let key_bytes = arguments.get(1).ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                //Handle remove request

                if let Err(error) = key_bytes {
                    return Err(error)
                }

                let key = String::from_utf8(key_bytes.unwrap().to_vec())?;

                let _result = kv_store.remove(key)?;

                //NOTE! If the result is not Ok(value), then error should propogate to kvs-server and the below should not execute right?
                //Send result back (encapsulate in function?)
                let response = OK_RESPONSE;

                stream.write(response)?;
                stream.flush()?;


            },
            _ => {
                //return error
                return Err(KvsError::CommandError("Command unrecognized".to_string()))
            }
        }

        Ok(())

    }

    //TODO! Return result or propogate error to client

}

// //NOTE! Deprecated since TcpStream::connect has this handling already
// fn parse_ip(ip_string: &String) -> Result<IpAddr> {
//     println!("Inside parse ip");
//     if ip_string.as_bytes().len() < 33 { //NOTE: Is there a better / more succint way to determine V4 vs. V6?
//         Ipv4Addr::from_str(&ip_string)
//             .map_err(|err| {err.into()})
//             .map(|ipv4| {IpAddr::V4(ipv4)})
//     } else {
//         Ipv6Addr::from_str(&ip_string)
//             .map_err(|err| {err.into()})
//             .map(|ipv6| {IpAddr::V6(ipv6)})
//     }
// }


pub trait KvsEngine{
    fn set(&mut self, key: String, value: String) -> Result<()>;

    fn get(&mut self, key: String) -> Result<Option<String>>;

    fn remove(&mut self, key: String) -> Result<()>;
}

///Primary struct is a KvStore containing a single HashMap
#[derive(Debug)]
pub struct KvStore {
    pub kv: HashMap<String, usize>, //Change to store log pointer
    pub directory_path: PathBuf,
    pub log_pointer: usize,
}

///Result wrapper to consolidate program errors
pub type Result<T> = std::result::Result<T, KvsError>;

///Custom errors for the program
#[derive(Debug)]
pub enum KvsError {
    Io(io::Error),
    Serde(serde_json::Error),
    Store(String),
    IpAddrParse(AddrParseError),
    CommandError(String),
    //TODO! Add IP Parse error and server error heres
}

impl fmt::Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KvsError::Io(err) => write!(f, "IO error: {}", err),
            KvsError::Serde(err) => write!(f, "Serde error: {}", err),
            KvsError::Store(err) => write!(f, "Store error {}", err),
            KvsError::IpAddrParse(err) => write!(f, "IP error {}", err),
            KvsError::CommandError(err) => write!(f, "Command error: {}", err),
        }
    }
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

impl From<net::AddrParseError> for KvsError {
    fn from(err: net::AddrParseError) -> KvsError {
        KvsError::IpAddrParse(err)
    }
}

impl From<std::string::FromUtf8Error> for KvsError {
    fn from(err: std::string::FromUtf8Error) -> KvsError {
        KvsError::CommandError(err.to_string())
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

    ///Open the KvStore at a given path. Return the KvStore
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        // println!("opening file");

        let directory: PathBuf = path.into();
        fs::create_dir_all(&directory)?;

        let full_path = directory.join("log.txt");
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
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.kv.insert(key.clone(), self.log_pointer);

        let command = Command::Set { key, value };

        let full_path = self.get_file_path();
        // println!("set full path: {:?}", full_path);

        // let file = File::open(full_path)?;

        let file = get_file(full_path)?;

        serde_json::to_writer(file, &command)?;

        // println!("Set write complete");

        self.log_pointer += 1;

        Ok(())
    }

    ///Remove a given key. Return an error if the key does not exist or is not removed successfully.
    fn remove(&mut self, key: String) -> Result<()> {
        let result = self.kv.remove(&key);

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

        self.log_pointer += 1;

        Ok(())
    }

    ///Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    fn get(&mut self, key: String) -> Result<Option<String>> {
        if self.kv.get(&key).cloned().is_none() {
            return Ok(None);
        }

        let log_pointer = self.kv.get(&key).unwrap();

        let full_path = self.get_file_path();
        // println!("set remove full path: {:?}", full_path);

        // println!("Opening file on disk as part of GET");
        // println!("full path of GET: {:?}", full_path);

        let file = get_file(full_path)?;

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
    for command in deserialized_commands.iter() {
        match command {
            Command::Set { key, value: _ } => {
                in_mem_kv.kv.insert(key.clone(), in_mem_kv.log_pointer);
                in_mem_kv.log_pointer += 1;
            }
            Command::Rm { key } => {
                in_mem_kv.kv.remove(key);
                in_mem_kv.log_pointer += 1;
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

    for (i, command) in deserialized_commands.iter().enumerate() {
        match command {
            Command::Rm { key: _ } => continue,
            Command::Set { key, value: _ } => {
                let pointer = in_mem_kv.kv.get(key);

                if pointer != None && *pointer.unwrap() == i {
                    new_disc.push(command.to_owned());
                    in_mem_kv.kv.insert(key.to_string(), i - removals);
                } else {
                    removals += 1;
                }
            }
        };
    }

    //reset pointer value
    in_mem_kv.log_pointer = new_disc.len();
}
