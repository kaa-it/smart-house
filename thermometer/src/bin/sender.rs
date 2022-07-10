use std::{
    error::Error,
    net::{SocketAddr, UdpSocket},
    thread,
    time::{Duration, Instant},
};

use clap::Parser;

/// Sender program for imitating the thermometer
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Address for receiver: <ip>:<port>
    #[clap(short, long, value_parser)]
    receiver: String,

    /// Address for binding: <ip>:<port>
    #[clap(short, long, value_parser)]
    bind: String,
}

fn main() {
    let args = Args::parse();

    println!("Receiver address: {}", args.receiver);
    println!("Binding to: {}", args.bind);

    let receiver = args
        .receiver
        .parse::<SocketAddr>()
        .expect("Failed to parse receiver address");

    let bind = args
        .bind
        .parse::<SocketAddr>()
        .expect("Failed to parse address for binding");

    let socket = UdpSocket::bind(bind).expect("Failed to bind socket");

    let temperature_generator = TemperatureGenerator::new(30.0, 5.0);

    println!("Start sending temperature from {bind} to {receiver}");

    loop {
        let temperature = temperature_generator.generate();

        let res = send_temperature(&socket, &receiver, temperature);

        if let Err(e) = res {
            println!("Failed to send temperature: {e}");
        }

        println!("Temperature: {temperature}");

        thread::sleep(Duration::from_millis(1500));
    }
}

fn send_temperature(
    socket: &UdpSocket,
    receiver: &SocketAddr,
    temperature: f64,
) -> Result<(), Box<dyn Error>> {
    let bytes = temperature.to_be_bytes();
    let mut sent_count = 0;

    while sent_count < 8 {
        let sent_bytes = socket.send_to(&bytes[sent_count..], receiver)?;

        println!("Sended");

        sent_count += sent_bytes;
    }

    Ok(())
}

/// Describes temperature generator for
/// range [`from` - `delta`; `from` + `delta`]
struct TemperatureGenerator {
    started: Instant,
    from: f64,
    delta: f64,
}

impl TemperatureGenerator {
    pub fn new(from: f64, delta: f64) -> Self {
        Self {
            started: Instant::now(),
            from,
            delta,
        }
    }

    pub fn generate(&self) -> f64 {
        let delta = Instant::now() - self.started;
        self.from + self.delta * (delta.as_secs_f64() / 2.0).cos()
    }
}
