use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::config::config;

mod error;
pub mod house;
pub mod room;
pub mod device;

pub use self::error::{Error, Result};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().DB_URL)
        .await
}

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool()
            .await
            .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
        Ok(ModelManager { db })
    }
}