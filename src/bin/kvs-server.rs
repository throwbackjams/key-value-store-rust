use clap::Parser;
use kvs::{KvStore, KvsError, Result, KvsEngine, KvsServer, parse_ip};
use tracing::{info, trace};
use tracing_subscriber;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;


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

fn main() -> Result<()> {
    
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

    let path = PathBuf::from("");
    let _verified_ip_address = parse_ip(&ip)?;

    info!("Listening on IP Address:Port: {}", ip);
    let listener = TcpListener::bind(ip)?; //TODO! Better error handling
    
    info!("Running kvs-server version: {}", env!("CARGO_PKG_VERSION"));
    info!("Engine used: {:?}", cli.engine.as_deref());

    for stream in listener.incoming() {
        let mut kv_store = KvStore::open(&path)?; //Q: What happens if two simultaneous connections occur? Race?
        let unwrapped_stream = stream?;
        info!("Connection established with stream: {:?}", unwrapped_stream);

        info!("Server handling request");
        KvsServer::handle_request(unwrapped_stream, kv_store);
    }

    Ok(())
    // println!("addr: {:?}", ip);
    // println!("engine: {:?}", cli.engine.as_deref());

}

