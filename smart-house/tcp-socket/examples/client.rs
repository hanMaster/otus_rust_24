use std::error::Error;

use tcp_socket::SocketConnector;

fn main() -> Result<(), Box<dyn Error>> {
    let client = tcp_socket::SocketClient::new("127.0.0.1:3456")?;
    client.turn_on()?;
    let state = client.get_socket_info()?;
    println!(
        "State: is_turned_on: {} power: {}",
        state.is_turned_on, state.power
    );
    client.turn_off()?;
    let state = client.get_socket_info()?;
    println!(
        "State: is_turned_on: {} power: {}",
        state.is_turned_on, state.power
    );
    Ok(())
}
