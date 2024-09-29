use std::collections::HashMap;

use crate::devices::SmartDevice;

pub type RoomName = String;

pub struct SmartHouse {
    name: String,
    devices: HashMap<RoomName, Vec<Box<dyn SmartDevice>>>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: HashMap::new(),
        }
    }

    pub fn room_list(&self) -> String {
        self.devices
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn add_device(&mut self, room_name: &str, device: impl SmartDevice + 'static) {
        let dev = Box::new(device);
        let mut list = self.devices.remove(room_name).unwrap_or_default();
        list.push(dev);
        self.devices.insert(room_name.to_string(), list);
    }

    pub fn get_room_devices(&self, room_name: &str) -> Option<&Vec<Box<dyn SmartDevice>>> {
        self.devices.get(room_name)
    }

    pub fn report(&self) -> String {
        let mut res: Vec<String> = vec![];

        for key in self.devices.keys() {
            res.push(format!("{}:", key));
            if let Some(vec) = self.devices.get(key) {
                let room_info = vec
                    .iter()
                    .map(|d| format!("\t {}", d.get_device_info()))
                    .collect::<Vec<_>>()
                    .join("\n");
                res.push(room_info);
            }
        }
        format!("Отчет о \"{}\"\n{}", self.name, res.join("\n"))
    }
}
