//! Module describes thermometer device for smart house

use std::{
    error::Error,
    fmt,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tokio::{net::UdpSocket, task::JoinHandle, time};
/// Describes smart thermometer
#[derive(Debug)]
pub struct Thermometer {
    temperature: Arc<Mutex<f64>>,
    stop: Arc<AtomicBool>,
    jh: Option<JoinHandle<()>>,
}

impl Thermometer {
    /// Creates new thermometer which receives data at given `recevier`
    pub async fn new(receiver: &str, sender: &str) -> Result<Self, Box<dyn Error>> {
        let receiver = receiver.parse::<SocketAddr>()?;
        let sender = sender.parse::<SocketAddr>()?;

        let socket = UdpSocket::bind(receiver).await?;

        let stop = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Mutex::new(0.0));

        let temperature_clone = temperature.clone();
        let stop_clone = stop.clone();

        let jh = tokio::spawn(async move {
            let socket = socket;
            loop {
                if stop_clone.load(Ordering::SeqCst) {
                    return;
                }

                let val = match Self::recv_temperature(&socket, &sender, stop_clone.clone()).await {
                    Err(err) => {
                        println!("Failed to receive temperature from sender: {err}");
                        0.0
                    }
                    Ok(None) => return,
                    Ok(Some(val)) => val,
                };

                *temperature_clone.lock().unwrap() = val;
            }
        });

        Ok(Self {
            temperature,
            stop,
            jh: Some(jh),
        })
    }

    /// Returns current temperature of the thermometer
    pub fn temperature(&self) -> f64 {
        *self.temperature.lock().unwrap()
    }

    async fn recv_temperature(
        socket: &UdpSocket,
        sender: &SocketAddr,
        stop: Arc<AtomicBool>,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        let mut buf = [0; 8];
        let mut recv_buf = [0; 8];
        let mut recv_count = 0;

        while recv_count < 8 {
            if stop.load(Ordering::SeqCst) {
                return Ok(None);
            }

            let (bytes_received, src_addr) = tokio::select! {
                resp = socket.recv_from(&mut recv_buf) => {
                    let (b, s) = resp?;
                    (b, s)
                },

                _ = time::sleep(time::Duration::from_secs(5)) => {
                    return Ok(None)
                }
            };

            if src_addr != *sender {
                continue;
            }

            buf[recv_count..recv_count + bytes_received]
                .copy_from_slice(&recv_buf[0..bytes_received]);

            recv_count += bytes_received;
        }

        Ok(Some(f64::from_be_bytes(buf)))
    }
}

impl fmt::Display for Thermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thermometer (temperature: {})", self.temperature())
    }
}

impl Drop for Thermometer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);

        let jh = self.jh.take().unwrap();

        _ = futures::executor::block_on(jh);
    }
}
