#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    // let cli = App:new("kvs")
    //     .about("CLI for a key value store in rust")
    //     .version("v0.0.1")
    //     .author("James Li")
    //     .arg(Arg::with_name("get"))
    //     .arg(Arg::with_name("set"))
    //     .arg(Arg::with_name("rm"))

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();


}