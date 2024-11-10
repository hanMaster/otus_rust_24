use std::error::Error;

use tcp_socket::SocketConnector;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = tcp_socket::SocketClient::new("127.0.0.1:3456");
    client.turn_on()?;
    client.turn_off()?;

    Ok(())
}
