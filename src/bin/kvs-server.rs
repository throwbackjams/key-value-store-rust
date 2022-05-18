use clap::Parser;
// use kvs::{KvStore, KvsError, Result, KvsEngine};
use tracing::{info, trace};
use tracing_subscriber;
use std::net::{TcpListener, TcpStream};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli{
    ///Set the IP Address at which the server will listen
    #[clap(short, long)]
    addr: Option<String>,
    ///Customize the engine used. Either kvs (built-in) or sled(plug-in)
    #[clap(short, long)]
    engine: Option<String>,
}

fn main() {
    
    let subscriber = tracing_subscriber::FmtSubscriber::new();

    let _result = tracing::subscriber::set_global_default(subscriber)
    .map_err(|_err| eprintln!("Unable to set global default subscriber"));

    trace!("Parsing IP Address and Engine options");

    let cli = Cli::parse();

    let ip: String;

    match cli.addr {
        Some(addr) => ip = addr,
        None => ip = String::from("127.0.0.1:4000"),
    };

    //TODO! Handle IP Address / Port Parsing & Error - OR have the handling logic sit in the kvs lib and import

    info!("Listening on IP Address:Port: {}", ip);
    let listener = TcpListener::bind(ip).unwrap(); //TODO! Better error handling
    
    info!("Running kvs-server version: {}", env!("CARGO_PKG_VERSION"));
    info!("Engine used: {:?}", cli.engine.as_deref());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info!("Connection established with stream: {:?}", stream);

        handle_connection(stream);
    }

    // println!("addr: {:?}", ip);
    // println!("engine: {:?}", cli.engine.as_deref());

}

fn handle_connection(mut stream: TcpStream) {
    info!("Inside handle_connection");
}