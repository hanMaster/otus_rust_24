use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tcp_socket::SocketServer::serve("127.0.0.1:3456")?;
    Ok(())
}
