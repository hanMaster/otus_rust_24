use crate::model::ModelManager;
use crate::Result;
use serde::{Deserialize, Serialize};
use tracing::log::info;

#[derive(Deserialize)]
pub struct RoomForAdd {
    pub house_id: i64,
    pub room_name: String,
}

#[derive(Serialize)]
pub struct RoomData {
    pub id: i64,
    pub house_id: i64,
    pub room_name: String,
}

impl ModelManager {
    pub async fn create_room(&self, house_id: i64, room_name: String) -> Result<RoomData> {
        info!("create room with name: {room_name}");
        let (id,): (i64,) =
            sqlx::query_as("INSERT INTO room (house_id, room_name) VALUES($1, $2) returning id")
                .bind(&house_id)
                .bind(&room_name)
                .fetch_one(&self.db)
                .await?;
        let room = self.read_room(id).await?;
        Ok(room)
    }

    pub async fn read_room(&self, id: i64) -> Result<RoomData> {
        let record = sqlx::query!("SELECT * FROM room WHERE id=$1", id)
            .fetch_one(&self.db)
            .await?;
        let room = RoomData {
            id,
            house_id: record.house_id,
            room_name: record.room_name,
        };

        Ok(room)
    }

    pub async fn rooms_list(&self, house_id: i64) -> Result<Vec<RoomData>> {
        info!("get room_list with house_id: {house_id}");
        let rows = sqlx::query!("SELECT * FROM room WHERE house_id=$1", house_id)
            .fetch_all(&self.db)
            .await?
            .iter()
            .map(|i| RoomData {
                id: i.id,
                house_id: i.house_id,
                room_name: i.room_name.clone(),
            })
            .collect();
        Ok(rows)
    }

    pub async fn delete_room(&self, id: i64) -> Result<RoomData> {
        info!("delete_room with id: {id}");
        let room = self.read_room(id).await?;
        sqlx::query!("DELETE FROM room WHERE id=$1", id)
            .execute(&self.db)
            .await?;
        Ok(room)
    }
}
