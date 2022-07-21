use crate::server::Server;
use clap::Parser;
use power_switch::power_switch::PowerSwitch;
use std::error::Error;

mod server;

/// Server program for serving the power switch
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Address for server: <ip>:<port>
    #[clap(short, long, value_parser)]
    address: String,

    /// Description of power switch
    #[clap(short, long, value_parser)]
    description: String,

    /// Turn the power switch on
    #[clap(short, long, default_value_t = 0)]
    enabled: u8,

    /// Power consumption of the power switch
    #[clap(short, long, default_value_t = 0.0)]
    power_consumption: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let power_switch = PowerSwitch::from_settings(
        &args.description,
        args.enabled.try_into()?,
        args.power_consumption,
    );

    println!("{power_switch}");

    let server = Server::new(args.address, power_switch).await?;

    server.run().await?;

    Ok(())
}
