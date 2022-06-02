use clap::Parser;
use kvs::client::KvsClient;
use kvs::error::Result;
use std::process;
// use tracing::{info};
// use tracing_subscriber;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1:4000";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli{
    #[clap(subcommand)]
    command: Command,
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
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    // let _result = tracing::subscriber::set_global_default(subscriber)
    // .map_err(|_err| eprintln!("Unable to set global default subscriber"));

    let cli = Cli::parse();

    match cli.command {
        Command::Set { key, value, addr } => {
            // info!("IP Address target: {:?}", addr);
            let message = String::from(format!("SET\n{}\n{}\n", key, value));
            // info!("Message request sent: {}", message);

            let string_response = KvsClient::connect_and_send_request(addr, message)?;

            // let string_response = String::from_utf8_lossy(&buffer_response[..]);

            let _trimmed_response = string_response.trim_start_matches('+');

            // info!("Response: {}", trimmed_response);

            // match result {
            //     Ok(()) => process::exit(0),
            //     Err(error) => {
            //         println!("Error: {}", error);
            //         process::exit(1);
            //     }
            // }

            Ok(())
        }
        Command::Get { key, addr } => {
            // info!("IP Address target: {:?}", addr);
            let message = String::from(format!("GET\n{}\n", key));
            // info!("Message request sent: {}", message);

            let string_response = KvsClient::connect_and_send_request(addr, message)?;

            let trimmed_response = string_response.trim_start_matches('+');

            println!("{}", trimmed_response);

            // info!("Response: {}", trimmed_response);

            // match result {
            //     Ok(Some(value)) => println!("{}", value),
            //     Ok(None) => println!("Key not found"),
            //     _ => unreachable!(),
            // }

            process::exit(0);
        }
        Command::Rm { key, addr } => {
            // info!("IP Address target: {:?}", addr);
            let message = String::from(format!("RM\n{}\n", key));
            // info!("Message request sent: {}", message);

            let string_response = KvsClient::connect_and_send_request(addr, message)?;

            let trimmed_response = string_response.trim_start_matches('+');

            if trimmed_response.starts_with("Key not found") {
                eprintln!("{}", trimmed_response);
                process::exit(1);

            }
            process::exit(0);
        }
    }
}
