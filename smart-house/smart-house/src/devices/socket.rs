use crate::devices::SmartDevice;

#[derive(Clone)]
pub struct Socket {
    name: String,
    description: String,
    is_on: bool,
    pwr: f64,
}

impl SmartDevice for Socket {
    fn get_device_name(&self) -> &str {
        &self.name
    }

    fn get_device_info(&self) -> String {
        format!(
            "{} {}, состояние: {}, мощность: {} Ватт",
            self.name,
            self.description,
            if self.is_on {
                "включена"
            } else {
                "выключена"
            },
            self.pwr
        )
    }
}

impl Socket {
    pub fn new() -> Self {
        Self {
            name: format!("розетка_{}", rand::random::<u16>()),
            description: String::new(),
            is_on: false,
            pwr: 0.0,
        }
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = String::from(description);
        self
    }

    pub fn switch_on(&mut self) -> &mut Self {
        self.is_on = true;
        self.pwr = 100.0;
        self
    }

    pub fn _switch_off(&mut self) {
        self.is_on = false;
        self.pwr = 0.0;
    }

    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}
