pub mod socket;
pub mod thermometer;

pub trait SmartDevice {
    fn get_device_name(&self) -> &str;
    fn get_device_info(&self) -> String;
}
