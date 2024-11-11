pub use client::SocketClient;
use error::Result;
pub use server::SocketServer;

mod client;
mod error;
mod server;

#[derive(Debug)]
pub struct SocketInfo {
    pub is_turned_on: bool,
    pub power: f64,
}

pub trait SocketConnector {
    fn get_socket_info(&self) -> Result<SocketInfo>;
    fn turn_on(&self) -> Result<()>;
    fn turn_off(&self) -> Result<()>;
}
