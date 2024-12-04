use std::error::Error;

use tcp_socket::SocketConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = tcp_socket::SocketClient::new("127.0.0.1:3456")?;
    client.turn_on().await?;
    let state = client.get_socket_info().await?;
    println!(
        "State: is_turned_on: {} power: {}",
        state.is_turned_on, state.power
    );
    client.turn_off().await?;
    let state = client.get_socket_info().await?;
    println!(
        "State: is_turned_on: {} power: {}",
        state.is_turned_on, state.power
    );
    Ok(())
}
