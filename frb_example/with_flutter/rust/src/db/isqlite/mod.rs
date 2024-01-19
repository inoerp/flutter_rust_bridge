pub mod execute;
pub mod select;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{self, Pool};
use std::collections::HashMap;
use std::error::Error;

pub const DB_URL: &str = "sqlite://assets/db/rikdata_erp_51.db";

pub struct ISqlite;

impl ISqlite {
    pub async fn get_connection_pool() -> Result<HashMap<String, Pool<sqlx::Sqlite>>, Box<dyn Error>> {
        let mut pools: HashMap<String, Pool<sqlx::Sqlite>> = HashMap::new();
        let connection_pool = SqlitePoolOptions::new()
            .min_connections(10)
            .connect(DB_URL)
            .await?;
        pools.insert("local".to_string(), connection_pool);
        Ok(pools)
    }
}


