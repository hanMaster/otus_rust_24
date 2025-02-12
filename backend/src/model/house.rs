use tracing::log::info;
use crate::error::Result;
use crate::model::ModelManager;

impl ModelManager {
    pub async fn create_house(&self, house_name: String) -> Result<i64> {
        info!("create house with name: {house_name}");
        let (id, ): (i64,) = sqlx::query_as("INSERT INTO house (house_name) VALUES($1) returning id")
            .bind(&house_name)
            .fetch_one(&self.db)
            .await?;
        info!("Inserted with id {id}");
        Ok(id)
    }
}
