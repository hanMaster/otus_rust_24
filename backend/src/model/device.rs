use crate::model::ModelManager;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::log::info;

#[derive(Deserialize)]
pub struct DeviceForAdd {
    pub device_type: String,
    pub device_name: String,
    pub device_ip: String,
}

#[derive(Serialize, Clone, FromRow)]
pub struct DeviceData {
    pub id: i64,
    pub room_id: i64,
    pub device_type: String,
    pub device_name: String,
    pub device_ip: String,
}

impl ModelManager {
    pub async fn create_device(
        &self,
        room_id: i64,
        data: DeviceForAdd,
    ) -> crate::Result<DeviceData> {
        info!("create device with name: {}", data.device_name);
        let (id,): (i64,) =
            sqlx::query_as("INSERT INTO device (room_id, device_type, device_name, device_ip) VALUES($1, $2, $3, $4) returning id")
                .bind(&room_id)
                .bind(&data.device_type)
                .bind(&data.device_name)
                .bind(&data.device_ip)
                .fetch_one(&self.db)
                .await?;
        let device = self.read_device(id).await?;
        Ok(device)
    }

    pub async fn read_device(&self, id: i64) -> crate::Result<DeviceData> {
        let device = sqlx::query_as("SELECT * FROM device WHERE id=$1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(device)
    }

    pub async fn devices_list(&self, room_id: i64) -> crate::Result<Vec<DeviceData>> {
        info!("get device_list with room_id: {room_id}");
        let rows = sqlx::query_as("SELECT * FROM device WHERE room_id=$1")
            .bind(room_id)
            .fetch_all(&self.db)
            .await?;
        Ok(rows)
    }

    pub async fn delete_device(&self, id: i64) -> crate::Result<DeviceData> {
        info!("delete_device with id: {id}");
        let device = self.read_device(id).await?;
        sqlx::query!("DELETE FROM device WHERE id=$1", id)
            .execute(&self.db)
            .await?;
        Ok(device)
    }

    pub async fn get_device_info(&self, device: &DeviceData) -> crate::Result<String> {
        match device.device_type.as_str() {
            "socket" => {
                // here we make async request to TCP Socket by device_ip
                Ok(format!(
                    "Socket - Name: {}, State: {}, Power: {} Watt",
                    device.device_name, "ON", 120
                ))
            }
            "thermometer" => {
                // here we make async request to UDP Thermometer by device_ip
                Ok(format!(
                    "Thermometer - Name: {}, Temperature: {} ℃",
                    device.device_name, 24
                ))
            }
            _ => Ok("unknown device".to_string()),
        }
    }
}
