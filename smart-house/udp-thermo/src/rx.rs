use crate::Result;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::mpsc;

pub struct Receiver {
    rx: mpsc::Receiver<f64>,
}

impl Receiver {
    pub async fn new<Addr>(addr: Addr) -> Result<Self>
    where
        Addr: ToSocketAddrs,
    {
        let (tx, rx) = mpsc::channel(1);
        let socket = UdpSocket::bind(addr).await?;
        tokio::spawn(async move {
            let mut buf = [0; 8];
            loop {
                if socket.recv(&mut buf).await.is_ok() {
                    let temp = f64::from_be_bytes(buf);
                    if tx.send(temp).await.is_err() {
                        eprintln!("failed to send temperature")
                    };
                }
            }
        });
        Ok(Self { rx })
    }
    pub async fn receive(&mut self) -> Option<f64> {
        self.rx.recv().await
    }
}
