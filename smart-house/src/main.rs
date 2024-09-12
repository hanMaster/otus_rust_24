use smart_house::thermometer::Thermometer;

use crate::smart_house::SmartDevice;
use crate::smart_house::socket::Socket;

mod smart_house;

fn main() {
    let thermometer = Thermometer::new();
    let mut socket = Socket::new();
    socket.set_description("Kitchen");
    socket.switch_on();
    println!("{}", thermometer.get_device_info());
    println!("{}", socket.get_device_info());
}
