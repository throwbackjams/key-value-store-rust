use clap::Parser;
use std::process;
use kvs::{KvStore, Result, KvsError};
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
    let path = PathBuf::from("tmp/.tmp");
    let mut in_mem_kv = KvStore::open(&path)?;

    println!("path: {:?}", in_mem_kv.directory_path);
    
    println!("In memory KV: {:?}", in_mem_kv.kv);
    
    let command = Command::parse();

    match command {
        Command::Set { key, value } => {
            // println!("Key value pair to be set {:?} : {:?}", key, value);

            let result = in_mem_kv.set(key, value);

            match result {
                Ok(()) => process::exit(0),
                Err(error) => {
                    println!("Error: {}", error);
                    process::exit(1);
                }

            }
        }
        Command::Get { key } => {
            
            let result = in_mem_kv.get(key)
                            .map_err(|error| { 
                                println!("{}", error);
                                process::exit(0);
                             })
                             .unwrap();

            

            process::exit(0);
        }
        Command::Rm { key } => {
            
            let _result = in_mem_kv
                            .remove(key)
                            .map_err(|error| {
                                if let KvsError::Store(err) = error {
                                println!("{}", err);
                                process::exit(1);
                            }
                            });
            // println!("CLI remove command - result: {:?}", result);

            process::exit(0);

        }
    }

    // todo!("implement result type")
}
