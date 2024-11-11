use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = tcp_socket::SocketServer::new("127.0.0.1:3456")?;
    server.serve()?;
    Ok(())
}
