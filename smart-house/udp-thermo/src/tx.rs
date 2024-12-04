use crate::Result;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

pub struct Transmitter<'a> {
    generator: ThreadRng,
    socket: UdpSocket,
    receiver_addr: &'a str,
}

impl<'a> Transmitter<'a> {
    pub fn new(receiver_addr: &'a str) -> Result<Self> {
        let _: SocketAddr = receiver_addr.parse()?;
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self {
            generator: rand::thread_rng(),
            socket,
            receiver_addr,
        })
    }

    pub fn serve(&mut self) {
        loop {
            match self.send() {
                Ok(_) => thread::sleep(Duration::from_secs(1)),
                Err(_) => {
                    println!("Receiver refused data");
                    break;
                }
            }
        }
    }

    fn send(&mut self) -> std::io::Result<usize> {
        let temp = self.gen_temp();
        println!("temp for sending: {temp}");
        self.socket.connect(self.receiver_addr)?;
        self.socket.send(temp.to_be_bytes().as_ref())
    }

    fn gen_temp(&mut self) -> f64 {
        self.generator.gen_range(10.0..50.0)
    }
}
