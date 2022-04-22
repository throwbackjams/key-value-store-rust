use clap::Parser;
use std::process;
#[clap(author, version, about)]
#[derive(Debug, Parser)]
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

fn main() {

    let command = Command::parse();

    println!("{:?}", command);

    match command {
        Command::Set{ key, value } => {
            eprintln!("unimplemented");

            println!("Key value pair to be set {:?} : {:?}", key, value);
            process::exit(1);
        },
        Command::Get{ key } => {
            eprintln!("unimplemented");

            println!("Key to be searched {:?}", key);
            process::exit(1);
        },
        Command::Rm { key } => {
            eprintln!("unimplemented");

            println!("Key to be removed {:?}", key);
            process::exit(1);
        },

    }

}