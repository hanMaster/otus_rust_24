use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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

    async fn turn(&self, need_turn_on: bool) -> Result<()> {
        let mut stream = TcpStream::connect(self.addr).await?;
        let cmd = if need_turn_on { b"cmd2" } else { b"cmd3" };
        stream.write_all(cmd).await?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"done" {
            return Err(Error::FailTurnSocket);
        }
        Ok(())
    }
}

impl SocketConnector for SocketClient<'_> {
    async fn get_socket_info(&self) -> Result<SocketInfo> {
        let mut stream = TcpStream::connect(self.addr).await?;
        stream.write_all(b"cmd1").await?;
        let mut buf = [0; 15];
        stream.read_exact(&mut buf).await?;
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

    async fn turn_on(&self) -> Result<()> {
        self.turn(true).await
    }

    async fn turn_off(&self) -> Result<()> {
        self.turn(false).await
    }
}
