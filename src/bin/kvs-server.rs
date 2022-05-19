use clap::Parser;
use kvs::{KvStore, KvsError, Result, KvsEngine, KvsServer};
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

    info!("Beginning Server listening on IP Address:Port: {}", ip);
    info!("Running kvs-server version: {}", env!("CARGO_PKG_VERSION"));
    info!("Engine used: {:?}", cli.engine.as_deref());
    
    KvsServer::listen_and_serve_requests(ip);
    
    Ok(())
    // println!("addr: {:?}", ip);
    // println!("engine: {:?}", cli.engine.as_deref());

}

