use std::io::{Read, Write};
use std::net::TcpStream;

use crate::{SocketConnector, SocketInfo};
use crate::error::{Error, Result};

pub struct SocketClient<'a> {
    addr: &'a str,
}

impl<'a> SocketClient<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    fn turn(&self, need_turn_on: bool) -> Result<()> {
        let mut stream = TcpStream::connect(self.addr)?;
        let cmd = if need_turn_on { b"cmd2" } else { b"cmd3" };
        stream.write_all(cmd)?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"done" {
            return Err(Error::FailTurnSocket);
        }
        Ok(())
    }
}

impl SocketConnector for SocketClient<'_> {
    fn get_socket_info(&self) -> Result<SocketInfo> {
        let mut stream = TcpStream::connect(self.addr)?;
        stream.write_all(b"cmd1")?;
        let mut buf = [0; 12];
        stream.read_exact(&mut buf)?;
        Ok(SocketInfo {
            is_turned_on: false,
            power: 0.0,
        })
    }

    fn turn_on(&self) -> Result<()> {
        self.turn(true)
    }

    fn turn_off(&self) -> Result<()> {
        self.turn(false)
    }
}
