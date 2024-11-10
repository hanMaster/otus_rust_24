pub use client::SocketClient;
use error::Result;
pub use server::SocketServer;

mod client;
mod error;
mod server;

pub struct SocketInfo {
    is_turned_on: bool,
    power: f64,
}

pub trait SocketConnector {
    fn get_socket_info(&self) -> Result<SocketInfo>;
    fn turn_on(&self) -> Result<()>;
    fn turn_off(&self) -> Result<()>;
}
