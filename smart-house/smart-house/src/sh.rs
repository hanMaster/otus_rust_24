use std::collections::BTreeMap;

use crate::devices::SmartDevice;
use crate::error::{Result, SmartHouseError};
pub type RoomName = String;
pub type DeviceList = Vec<Box<dyn SmartDevice>>;

pub struct SmartHouse {
    name: String,
    devices: BTreeMap<RoomName, DeviceList>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: BTreeMap::new(),
        }
    }
    /// Возвращает список комнат дома в виде строки через запятую
    pub fn room_list(&self) -> String {
        self.devices
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Добавление в хранилище устройства в комнате
    pub fn add_device(&mut self, room_name: &str, device: impl SmartDevice + 'static) {
        let dev = Box::new(device);
        let mut list = self.devices.remove(room_name).unwrap_or_default();
        list.push(dev);
        self.devices.insert(room_name.to_string(), list);
    }

    /// Возвращает массив устройств по названию помещения
    pub fn get_room_devices(&self, room_name: &str) -> Result<&DeviceList> {
        let list = self.devices.get(room_name);
        if list.is_none() {
            return Err(SmartHouseError::InvalidName);
        }
        Ok(list.unwrap())
    }

    /// Формирует отчет по всем помещениям и устройствам дома
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

#[cfg(test)]
mod tests {
    use crate::devices::thermometer::Thermometer;

    use super::*;

    #[test]
    fn house_has_name() {
        let house = SmartHouse::new("Загородный дом");
        assert_eq!(house.name, "Загородный дом")
    }

    #[test]
    fn room_list() {
        let mut house = SmartHouse::new("Загородный дом");
        house.add_device("Кухня", Thermometer::new());
        house.add_device("Столовая", Thermometer::new());
        house.add_device("Спальня", Thermometer::new());
        assert_eq!(house.room_list(), "Кухня, Спальня, Столовая");
    }

    #[test]
    fn room_devices_list() {
        let mut house = SmartHouse::new("Загородный дом");
        house.add_device("Кухня", Thermometer::new());
        house.add_device("Кухня", Thermometer::new());
        house.add_device("Столовая", Thermometer::new());
        let list = house.get_room_devices("Кухня").unwrap();
        assert_eq!(list.len(), 2);
        let list = house.get_room_devices("Столовая").unwrap();
        assert_eq!(list.len(), 1);
    }
}
