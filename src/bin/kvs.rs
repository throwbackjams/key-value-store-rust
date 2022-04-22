#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::process;

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        None => {
            eprintln!("Must contain valid subcommand or arguments");
            process::exit(1);
        },
        Some(("set", argmatches)) => {
            eprintln!("unimplemented");

            let key = argmatches.value_of("key").unwrap();
            let value = argmatches.value_of("value").unwrap();

            println!("Key value pair to be set {:?} : {:?}", key, value);
            process::exit(1);
        },
        Some(("get", argmatches)) => {
            eprintln!("unimplemented");

            let key = argmatches.value_of("key").unwrap();

            println!("Key to be searched {:?}", key);
            process::exit(1);
        },
        Some(("rm", argmatches)) => {
            eprintln!("unimplemented");

            let key = argmatches.value_of("key").unwrap();

            println!("Key to be removed {:?}", key);
            process::exit(1);
        },
        _ => unreachable!(),
    }


}