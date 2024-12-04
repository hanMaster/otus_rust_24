use std::error::Error;
use udp_thermo::Receiver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut receiver = Receiver::new("127.0.0.1:8081").await?;
    loop {
        if let Some(temp) = receiver.receive().await {
            println!("temp received: {}", temp);
        }
    }
}
