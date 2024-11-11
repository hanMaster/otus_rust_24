use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let mut server = tcp_socket::SocketServer::new("127.0.0.1:3456")?;
    server.serve()?;
    Ok(())
}
