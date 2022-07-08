use power_switch::command::Command;
use power_switch::response::Response;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(server_address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_address)?;
        Ok(Self { stream })
    }

    pub fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; 9];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}
