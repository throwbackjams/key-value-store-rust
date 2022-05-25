use clap::Parser;
use kvs::server::KvsServer;
use kvs::error::Result;
use tracing::{ info, trace };
use tracing_subscriber;
use kvs::utils::KVS_CODE;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1:4000";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli{
    ///Set the IP Address at which the server will listen
    #[clap(short, long, default_value_t = String::from(DEFAULT_IP_ADDRESS).to_string())]
    addr: String,
    ///Customize the engine used. Either kvs (built-in) or sled(plug-in)
    #[clap(short, long, default_value_t = String::from_utf8_lossy(KVS_CODE).to_string())]
    engine: String,
}

fn main() -> Result<()> {
    
    let subscriber = tracing_subscriber::FmtSubscriber::new();

    let _result = tracing::subscriber::set_global_default(subscriber)
    .map_err(|_err| eprintln!("Unable to set global default subscriber"));

    trace!("Parsing IP Address and Engine options");

    let cli = Cli::parse();

    info!("Beginning Server listening on IP Address:Port: {}", cli.addr);
    info!("Running kvs-server version: {}", env!("CARGO_PKG_VERSION"));
    info!("Engine used: {:?}", cli.engine);

    eprintln!("Beginning Server listening on IP Address:Port: {}", cli.addr);
    eprintln!("Running kvs-server CARGO_PKG_VERSION: {}", env!("CARGO_PKG_VERSION"));
    eprintln!("Engine used: {:?}", cli.engine);
    
    KvsServer::route_request(cli.addr, cli.engine)?;
    
    Ok(())

}

