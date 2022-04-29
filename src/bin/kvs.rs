use clap::Parser;
use std::process;
use kvs::{KvStore, Result};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Command {
    ///Set the value of a string key to a string
    Set {
        #[clap(required = true)]
        key: String,
        #[clap(required = true)]
        value: String,
    },
    ///Get the string value of a given string key
    Get {
        #[clap(required = true)]
        key: String,
    },
    ///Removes a given key
    Rm {
        #[clap(required = true)]
        key: String,
    },
}

fn main() -> Result<()> {
    let path = PathBuf::from(r"log.txt");
    let mut in_mem_kv = KvStore::open(&path)?;
    
    println!("In memory KV: {:?}", in_mem_kv.kv);
    
    let command = Command::parse();

    match command {
        Command::Set { key, value } => {
            // println!("Key value pair to be set {:?} : {:?}", key, value);

            in_mem_kv.set(key, value)?;

            process::exit(0);
        }
        Command::Get { key } => {
            
            let result = in_mem_kv.get(key);

            println!("get result: {:?}", result);

            process::exit(0);
        }
        Command::Rm { key } => {
            
            in_mem_kv.remove(key)?;

            process::exit(0);

        }
    }

    // todo!("implement result type")
}
