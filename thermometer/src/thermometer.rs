//! Module describes thermometer device for smart house

use std::{
    error::Error,
    fmt,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
/// Describes smart thermometer
#[derive(Debug, Clone)]
pub struct Thermometer {
    temperature: Arc<Mutex<f64>>,
    stop: Arc<AtomicBool>,
}

impl Thermometer {
    /// Creates new thermometer which receives data at given `recevier`
    pub fn new(receiver: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(receiver)?;
        socket.set_read_timeout(Some(Duration::from_secs(3)))?;

        let stop = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Mutex::new(0.0));

        let temperature_clone = temperature.clone();
        let stop_clone = stop.clone();

        thread::spawn(move || loop {
            if stop_clone.load(Ordering::SeqCst) {
                return;
            }

            let mut buf = [0; 8];
            if let Err(err) = socket.recv_from(&mut buf) {
                println!("Failed to receive temperature from sender: {err}");
            }

            let val = f64::from_be_bytes(buf);
            *temperature_clone.lock().unwrap() = val;
        });

        Ok(Self { temperature, stop })
    }

    /// Returns current temperature of the thermometer
    pub fn temperature(&self) -> f64 {
        *self.temperature.lock().unwrap()
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
    }
}
