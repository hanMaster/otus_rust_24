use std::io::{Read, Write};
use std::net::{TcpListener, ToSocketAddrs};
use std::str;
use tracing::{error, info};

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
        info!("Listening...");
        loop {
            let (mut stream, _) = self.listener.accept()?;
            let mut buf = vec![0; 4];
            stream.read_exact(&mut buf)?;
            if let Ok(cmd) = str::from_utf8(&buf) {
                let res = self.process_command(cmd);
                stream.write_all(&res)?
            } else {
                error!("Bad data received")
            }
        }
    }

    fn process_command(&mut self, cmd: &str) -> Vec<u8> {
        match cmd {
            "cmd1" => {
                info!("Status requested");
                let prefix = b"state:".as_slice();
                let pwr_bytes = self.power.to_be_bytes();
                let pwr_bytes_slice = pwr_bytes.as_slice();
                let state_array = &[self.is_turned_on as u8];
                [prefix, state_array, pwr_bytes_slice].concat()
            }
            "cmd2" => {
                info!("Turn on requested");
                self.is_turned_on = true;
                self.power = 12.5;
                b"done".into()
            }
            "cmd3" => {
                info!("Turn off requested");
                self.is_turned_on = false;
                self.power = 0.0;
                b"done".into()
            }
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_command1() {
        let mut srv = SocketServer::new("127.0.0.1:5000").unwrap();
        let _ = srv.process_command("cmd2");
        let res = srv.process_command("cmd1");
        assert_eq!(*b"state:", res[..6]);
        assert_eq!(1, res[6]);
        assert_eq!(12.5_f64.to_be_bytes(), res[7..]);
    }

    #[test]
    fn test_process_command2() {
        let mut srv = SocketServer::new("127.0.0.1:5001").unwrap();
        let res = srv.process_command("cmd2");
        assert_eq!(*b"done", *res);
        assert_eq!(true, srv.is_turned_on);
        assert_eq!(12.5, srv.power);
    }

    #[test]
    fn test_process_command3() {
        let mut srv = SocketServer::new("127.0.0.1:5002").unwrap();
        let res = srv.process_command("cmd3");
        assert_eq!(*b"done", *res);
        assert_eq!(false, srv.is_turned_on);
        assert_eq!(0.0, srv.power);
    }
}
