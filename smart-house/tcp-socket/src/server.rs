use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::str;

use crate::error::Result;

pub struct SocketServer {
    is_turned_on: bool,
    power: f64,
    listener: TcpListener,
}

impl SocketServer {
    pub fn new<Addr>(addr: Addr) -> Result<Self>
    where
        Addr: ToSocketAddrs,
    {
        let listener = TcpListener::bind(addr)?;
        Ok(Self {
            is_turned_on: false,
            power: 0.0,
            listener,
        })
    }

    pub fn serve(&mut self) -> Result<()> {
        println!("Listening...");
        loop {
            let (mut stream, _) = self.listener.accept()?;
            let mut buf = vec![0; 4];
            stream.read_exact(&mut buf)?;
            if let Ok(cmd) = str::from_utf8(&buf) {
                self.process_data(cmd, stream)?
            } else {
                eprintln!("Bad data received")
            }
        }
    }

    fn process_data(&mut self, data: &str, mut stream: TcpStream) -> Result<()> {
        match data {
            "cmd1" => {
                println!("Status requested");
                let prefix = b"state:".as_slice();
                let pwr_bytes = self.power.to_be_bytes();
                let pwr_bytes_slice = pwr_bytes.as_slice();
                let state_array = &[self.is_turned_on as u8];
                let response = [prefix, state_array, pwr_bytes_slice].concat();
                stream.write_all(&response)?;
            }
            "cmd2" => {
                println!("Turn on requested");
                self.is_turned_on = true;
                self.power = 12.5; // Здесь можно было использовать random, но все равно это будут моковые данные...
                stream.write_all(b"done")?;
            }
            "cmd3" => {
                println!("Turn off requested");
                self.is_turned_on = false;
                self.power = 0.0;
                stream.write_all(b"done")?;
            }
            _ => eprintln!("Unknown command received"),
        }
        Ok(())
    }
}
