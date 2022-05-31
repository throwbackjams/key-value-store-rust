// #![deny(missing_docs)]
//!An implementation of a key value store in Rust
pub use client::KvsClient;
pub use server::KvsServer;
pub use utils::{KVS_FILE_NAME};
pub use error::Result;
pub use engines::{KvsEngine, KvStore};
pub use thread_pool::*;

pub mod utils;
pub mod client;
pub mod server;
pub mod engines;
pub mod error;
pub mod thread_pool;

