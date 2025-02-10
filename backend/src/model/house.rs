use crate::model::ModelManager;
use crate::error::Result;

impl ModelManager {
    pub async fn create_house(&self, house_name: String) -> Result<()> {
        println!("create house {house_name}");
        // let id = sqlx::query!("INSERT INTO house (house_name) VALUES(?)", &house_name)
        //     .execute(&self.db)
        //     .await?
        //     .last_insert_id();
        Ok(())
    }
}