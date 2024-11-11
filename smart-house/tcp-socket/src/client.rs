use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

use crate::error::{Error, Result};
use crate::{SocketConnector, SocketInfo};

pub struct SocketClient<'a> {
    addr: &'a str,
}

impl<'a> SocketClient<'a> {
    pub fn new(addr: &'a str) -> Result<Self> {
        let _: SocketAddr = addr.parse()?;
        Ok(Self { addr })
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
        let mut buf = [0; 15];
        stream.read_exact(&mut buf)?;
        if &buf[0..6] != b"state:" {
            return Err(Error::FailGetInfo);
        }
        let is_turned_on = buf[6] == 1;
        let pwr_buf: [u8; 8] = buf[7..].try_into()?;
        let power = f64::from_be_bytes(pwr_buf);
        Ok(SocketInfo {
            is_turned_on,
            power,
        })
    }

    fn turn_on(&self) -> Result<()> {
        self.turn(true)
    }

    fn turn_off(&self) -> Result<()> {
        self.turn(false)
    }
}
