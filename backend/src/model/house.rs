use crate::error::Result;
use crate::model::ModelManager;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::log::info;

#[derive(Deserialize)]
pub struct HouseForAdd {
    pub name: String,
}

#[derive(Serialize, FromRow)]
pub struct HouseData {
    pub id: i64,
    pub house_name: String,
}

#[derive(Serialize)]
pub struct RoomForReport {
    room_name: String,
    device_info: Vec<String>,
}

#[derive(Serialize)]
pub struct HouseReport {
    house_name: String,
    rooms: Vec<RoomForReport>,
}

impl ModelManager {
    pub async fn create_house(&self, house_name: String) -> Result<HouseData> {
        info!("create house with name: {house_name}");
        let (id,): (i64,) =
            sqlx::query_as("INSERT INTO house (house_name) VALUES($1) returning id")
                .bind(&house_name)
                .fetch_one(&self.db)
                .await?;
        let house = self.read_house(id).await?;
        Ok(house)
    }

    pub async fn read_house(&self, id: i64) -> Result<HouseData> {
        let house = sqlx::query_as("SELECT * FROM house WHERE id=$1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(house)
    }

    pub async fn houses_list(&self) -> Result<Vec<HouseData>> {
        info!("get house_list");
        let rows = sqlx::query_as("SELECT * FROM house")
            .fetch_all(&self.db)
            .await?;
        Ok(rows)
    }

    pub async fn delete_house(&self, id: i64) -> Result<HouseData> {
        info!("delete_house with id: {id}");
        let house = self.read_house(id).await?;
        sqlx::query!("DELETE FROM house WHERE id=$1", id)
            .execute(&self.db)
            .await?;
        Ok(house)
    }

    pub async fn house_report(&self, id: i64) -> Result<HouseReport> {
        let house = self.read_house(id).await?;
        let rooms_list = self.rooms_list(id).await?;
        let mut rooms: Vec<RoomForReport> = vec![];

        for room in rooms_list {
            let devices = self.devices_list(room.id).await?;
            let mut device_info: Vec<String> = vec![];
            for device in devices {
                let info = self.get_device_info(&device).await?;
                device_info.push(info);
            }
            rooms.push(RoomForReport {
                room_name: room.room_name,
                device_info,
            })
        }

        Ok(HouseReport {
            house_name: house.house_name,
            rooms,
        })
    }
}
