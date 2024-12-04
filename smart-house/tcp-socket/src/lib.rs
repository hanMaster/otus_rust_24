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
    fn get_socket_info(&self) -> impl std::future::Future<Output = Result<SocketInfo>> + Send;
    fn turn_on(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    fn turn_off(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}
