use power_switch::command::Command;
use power_switch::response::Response;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn new(server_address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_address).await?;
        Ok(Self { stream })
    }

    pub async fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()]).await?;
        let mut buffer = [0u8; 9];
        self.stream.read_exact(&mut buffer).await?;
        Ok(buffer.into())
    }
}
