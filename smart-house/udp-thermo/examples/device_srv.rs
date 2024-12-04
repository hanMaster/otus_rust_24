use std::error::Error;
use udp_thermo::Transmitter;

fn main() -> Result<(), Box<dyn Error>> {
    let mut transmitter = Transmitter::new("127.0.0.1:8081")?;
    transmitter.serve();
    Ok(())
}
