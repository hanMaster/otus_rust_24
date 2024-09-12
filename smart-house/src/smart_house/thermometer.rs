use crate::smart_house::SmartDevice;

pub struct Thermometer {
    temp: f64,
}

impl SmartDevice for Thermometer {
    fn get_device_info(&self) -> String {
        format!(
            "DeviceType: Thermometer, current temperature: {}",
            self.temp
        )
    }
}

impl Thermometer {
    pub fn new() -> Self {
        Self { temp: 20.0 }
    }

    pub fn _get_temp(&self) -> f64 {
        self.temp
    }
}
