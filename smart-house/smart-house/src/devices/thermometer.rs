use crate::devices::SmartDevice;

pub struct Thermometer {
    name: String,
    temp: f64,
}

impl SmartDevice for Thermometer {
    fn get_device_name(&self) -> &str {
        &self.name
    }

    fn get_device_info(&self) -> String {
        format!("{}, текущая температура: {} °С", self.name, self.temp)
    }
}

impl Thermometer {
    pub fn new() -> Self {
        Self {
            name: format!("термометр_{}", rand::random::<u16>()),
            temp: 20.0,
        }
    }

    pub fn _get_temp(&self) -> f64 {
        self.temp
    }
}

impl Default for Thermometer {
    fn default() -> Self {
        Self::new()
    }
}
