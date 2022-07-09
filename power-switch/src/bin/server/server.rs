use power_switch::power_switch::PowerSwitch;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    tcp: TcpListener,
    power_switch: Arc<Mutex<PowerSwitch>>,
}

impl Server {
    pub fn new(addrs: impl ToSocketAddrs, power_switch: PowerSwitch) -> Result<Self, &'static str> {
        let tcp = TcpListener::bind(addrs).map_err(|_| "Failed to bind tcp listener")?;
        Ok(Self {
            tcp,
            power_switch: Arc::new(Mutex::new(power_switch)),
        })
    }

    pub fn run(&self) -> Result<(), &str> {
        for connection in self.tcp.incoming() {
            let stream = match connection {
                Ok(stream) => stream,
                Err(e) => {
                    println!("Can't receive connection: {e}");
                    continue;
                }
            };

            let peer = stream
                .peer_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "unknown".into());

            println!("Client connected: {peer}");

            let power_switch = self.power_switch.clone();

            thread::spawn(move || {
                match handle_connection(stream, power_switch) {
                    Ok(_) => println!("Client disconnected: {peer}"),
                    Err(e) => println!("Client {peer}: {e}"),
                };
            });
        }

        Ok(())
    }
}

fn handle_connection(
    mut stream: TcpStream,
    power_switch: Arc<Mutex<PowerSwitch>>,
) -> Result<(), &'static str> {
    let mut in_buffer = [0u8];
    while stream.read_exact(&mut in_buffer).is_ok() {
        let response = {
            let mut ps = power_switch.lock().unwrap();
            ps.process_command(in_buffer[0].into())
        };

        let response_buf: [u8; 9] = response.into();
        if stream.write_all(&response_buf).is_err() {
            return Err("Failed to send response");
        }
    }
    Ok(())
}
