use std::fmt;
use std::net::{ self, AddrParseError };
use std::io;
use std::sync::PoisonError;

///Result wrapper to consolidate program errors
pub type Result<T> = std::result::Result<T, KvsError>;

///Custom errors for the program
#[derive(Debug)]
pub enum KvsError{
    Io(io::Error),
    Serde(serde_json::Error),
    Store(String),
    IpAddrParse(AddrParseError),
    CommandError(String),
    SledError(sled::Error),
    ThreadPoolError(String),
    // SyncError(PoisonError<T>),
}

impl fmt::Display for KvsError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KvsError::Io(err) => write!(f, "IO error: {}", err),
            KvsError::Serde(err) => write!(f, "Serde error: {}", err),
            KvsError::Store(err) => write!(f, "Store error {}", err),
            KvsError::IpAddrParse(err) => write!(f, "IP error {}", err),
            KvsError::CommandError(err) => write!(f, "Command error: {}", err),
            KvsError::SledError(err) => write!(f, "Sled error: {}", err),
            KvsError::ThreadPoolError(err) => write!(f, "Thread pool error:{}", err),
            // KvsError::SyncError(err) => write!(f, "Sync Error: {}", err),
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

impl From<sled::Error> for KvsError{
    fn from(err: sled::Error) -> KvsError {
        KvsError::SledError(err)
    }
}

// impl <T> From<sync::PoisonError<T>> for KvsError<T> {
//     fn from(err: PoisonError<T>) -> KvsError<T> {
//         KvsError::SyncError(err)
//     }
// }