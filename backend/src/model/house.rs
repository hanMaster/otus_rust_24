use crate::error::Result;
use crate::model::ModelManager;
use serde::{Deserialize, Serialize};
use tracing::log::info;

#[derive(Deserialize)]
pub struct HouseForAdd {
    pub name: String,
}

#[derive(Serialize)]
pub struct HouseData {
    pub id: i64,
    pub house_name: String,
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
        let record = sqlx::query!("SELECT * FROM house WHERE id=$1", id)
            .fetch_one(&self.db)
            .await?;
        let house = HouseData {
            id,
            house_name: record.house_name,
        };

        Ok(house)
    }

    pub async fn houses_list(&self) -> Result<Vec<HouseData>> {
        info!("get house_list");
        let rows = sqlx::query!("SELECT * FROM house")
            .fetch_all(&self.db)
            .await?
            .iter()
            .map(|i| HouseData {
                id: i.id,
                house_name: i.house_name.clone(),
            })
            .collect();
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
}
