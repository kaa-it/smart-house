use power_switch::power_switch::PowerSwitch;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct Server {
    tcp: TcpListener,
    power_switch: Arc<Mutex<PowerSwitch>>,
}

impl Server {
    pub async fn new(
        addrs: impl ToSocketAddrs,
        power_switch: PowerSwitch,
    ) -> Result<Self, &'static str> {
        let tcp = TcpListener::bind(addrs)
            .await
            .map_err(|_| "Failed to bind tcp listener")?;
        Ok(Self {
            tcp,
            power_switch: Arc::new(Mutex::new(power_switch)),
        })
    }

    pub async fn run(&self) -> Result<(), &str> {
        loop {
            let (stream, peer_addr) = match self.tcp.accept().await {
                Ok(stream) => stream,
                Err(e) => {
                    println!("Can't receive connection: {e}");
                    continue;
                }
            };

            let peer = peer_addr.to_string();

            println!("Client connected: {peer}");

            let power_switch = self.power_switch.clone();

            tokio::spawn(async move {
                match handle_connection(stream, power_switch).await {
                    Ok(_) => println!("Client disconnected: {peer}"),
                    Err(e) => println!("Client {peer}: {e}"),
                };
            });
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    power_switch: Arc<Mutex<PowerSwitch>>,
) -> Result<(), &'static str> {
    let mut in_buffer = [0u8];
    while stream.read_exact(&mut in_buffer).await.is_ok() {
        let response = {
            let mut ps = power_switch.lock().unwrap();
            ps.process_command(in_buffer[0].into())
        };

        let response_buf: [u8; 9] = response.into();
        if stream.write_all(&response_buf).await.is_err() {
            return Err("Failed to send response");
        }
    }
    Ok(())
}
