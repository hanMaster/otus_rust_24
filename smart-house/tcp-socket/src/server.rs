use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::str;

use crate::error::Result;

pub struct SocketServer {}

impl SocketServer {
    pub fn serve<Addr>(addr: Addr) -> Result<()>
    where
        Addr: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr)?;
        println!("Listening...");
        loop {
            let (mut stream, _) = listener.accept()?;
            let mut buf = vec![0; 4];
            stream.read_exact(&mut buf)?;
            if let Ok(cmd) = str::from_utf8(&buf) {
                Self::process_data(cmd, stream)?
            } else {
                eprintln!("Bad data received")
            }
        }
    }

    fn process_data(data: &str, mut stream: TcpStream) -> Result<()> {
        println!("data: {}", data);
        stream.write_all(b"done")?;
        Ok(())
    }
}
