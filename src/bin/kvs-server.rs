use clap::Parser;
use kvs::{Result, KvsServer};
use tracing::{info, trace, error};
use tracing_subscriber;


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

    let mut ip: String;

    match cli.addr {
        Some(addr) => ip = addr,
        None => ip = String::from("127.0.0.1:4000"),
    };

    let mut engine: String;
    
    match cli.engine {
        Some(eng) => engine = eng,
        None => engine = "kvs".to_string(),
    };

    info!("Beginning Server listening on IP Address:Port: {}", ip);
    info!("Running kvs-server CARGO_PKG_VERSION: {}", env!("CARGO_PKG_VERSION"));
    info!("Engine used: {:?}", engine);

    eprintln!("Beginning Server listening on IP Address:Port: {}", ip);
    eprintln!("Running kvs-server CARGO_PKG_VERSION: {}", env!("CARGO_PKG_VERSION"));
    eprintln!("Engine used: {:?}", engine);
    
    KvsServer::route_request(ip, engine)?;
    
    Ok(())
    // println!("addr: {:?}", ip);
    // println!("engine: {:?}", cli.engine.as_deref());

}

