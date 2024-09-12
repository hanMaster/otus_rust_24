use crate::smart_house::SmartDevice;

pub struct Socket {
    description: String,
    is_on: bool,
    pwr: f64,
}

impl SmartDevice for Socket {
    fn get_device_info(&self) -> String {
        format!(
            "DeviceType: Socket, description: {}, current state: {}, current power value: {} Watt",
            self.description,
            if self.is_on { "active" } else { "inactive" },
            self.pwr
        )
    }
}

impl Socket {
    pub fn new() -> Self {
        Self {
            description: String::new(),
            is_on: false,
            pwr: 0.0,
        }
    }

    pub fn set_description(&mut self, description: &str) -> &mut Socket {
        self.description = String::from(description);
        self
    }

    pub fn switch_on(&mut self) {
        self.is_on = true;
        self.pwr = 100.0;
    }

    pub fn _switch_off(&mut self) {
        self.is_on = false;
        self.pwr = 0.0;
    }
}
