use clap::Parser;
use kvs::{KvStore, KvsError, Result, KvsEngine};
use std::path::PathBuf;
use std::process;
use tracing::{info, trace};
use tracing_subscriber;
use std::net::{IpAddr, TcpListener, TcpStream, Ipv4Addr, Ipv6Addr};

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1:4000";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli{
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Command {
    ///Set the value of a string key to a string
    Set {
        ///The key to be set
        #[clap(required = true)]
        key: String,
        ///The value to be set
        #[clap(required = true)]
        value: String,
        ///Optional IP:PORT target
        #[clap(short, long, default_value_t = String::from(DEFAULT_IP_ADDRESS))]
        addr: String,
    },
    ///Get the string value of a given string key
    Get {
        #[clap(required = true)]
        key: String,
        ///Optional IP:PORT target
        #[clap(short, long, default_value_t = String::from(DEFAULT_IP_ADDRESS))]
        addr: String,
    },
    ///Removes a given key
    Rm {
        #[clap(required = true)]
        key: String,
        ///Optional IP:PORT target
        #[clap(short, long, default_value_t = String::from(DEFAULT_IP_ADDRESS))]
        addr: String,
    },
}


fn main() -> Result<()> {
    let path = PathBuf::from("");
    let mut in_mem_kv = KvStore::open(&path)?;

    // println!("path: {:?}", in_mem_kv.directory_path);

    // println!("In memory KV: {:?}", in_mem_kv.kv);

    let subscriber = tracing_subscriber::FmtSubscriber::new();

    let _result = tracing::subscriber::set_global_default(subscriber)
    .map_err(|_err| eprintln!("Unable to set global default subscriber"));

    let cli = Cli::parse();

    match cli.command {
        Some(Command::Set { key, value, addr }) => {
            // println!("Key value pair to be set {:?} : {:?}", key, value);

            //TODO! Handle IP Address / Port Parsing & Error - OR have the handling logic sit in the kvs lib and import

            // let ip2 = IpAddr::from_str(addr)?;
            // println!("ip2: {:?}", ip2);
            
            // let ip: IpAddr;

            // match addr {
            //     Some(addr) => ip = IpAddr::from(addr),
            //     None => ip = IpAddr::from("127.0.0.1:4000")?,
            // };
            
            info!("IP Address target: {:?}", addr);

            let result = in_mem_kv.set(key, value);


            match result {
                Ok(()) => process::exit(0),
                Err(error) => {
                    println!("Error: {}", error);
                    process::exit(1);
                }
            }
        }
        Some(Command::Get { key, addr }) => {
            let result = in_mem_kv.get(key);
            // .map_err(|error| {
            //     if let KvsError::Store(err) = error {
            //         println!("{}", err);
            //         process::exit(0);
            //     }
            //  })
            //  .map(|result| {
            //      println!("{}", result.unwrap());
            //  });

            info!("IP Address target: {:?}", addr);

            match result {
                Ok(Some(value)) => println!("{}", value),
                Ok(None) => println!("Key not found"),
                _ => unreachable!(),
            }

            process::exit(0);
        }
        Some(Command::Rm { key, addr }) => {
            
            info!("IP Address target: {:?}", addr);

            let _result = in_mem_kv.remove(key).map_err(|error| {
                if let KvsError::Store(err) = error {
                    println!("{}", err);
                    process::exit(1);
                }
            });
            // println!("CLI remove command - result: {:?}", result);

            process::exit(0);
        }
        None => {Ok(())}
    }

    // todo!("implement result type")
}
