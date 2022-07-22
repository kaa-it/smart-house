use std::time::Duration;

use clap::Parser;
use thermometer::thermometer::Thermometer;
use tokio::time::sleep;

/// Sender program for imitating the thermometer
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Address for receive datagrams from thermometer: <ip>:<port>
    #[clap(short, long, value_parser)]
    receiver: String,

    /// Address of sender of thermometer data
    #[clap(short, long, value_parser)]
    sender: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let thermometer = Thermometer::new(&args.receiver, &args.sender)
        .await
        .unwrap();
    for _ in 0..88 {
        sleep(Duration::from_secs(2)).await;
        let temperature = thermometer.temperature();
        println!("The temperature is {temperature}");
    }
}
