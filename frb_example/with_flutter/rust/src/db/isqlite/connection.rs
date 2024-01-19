// use std::error::Error;

// use super::DB_URL;
// use sqlx::{self, SqlitePool, Pool};
// pub async fn get_connection_pool(
//     sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
// ) -> Result<&Pool<sqlx::Sqlite>, Box<dyn Error>> {
//     if let Some(pool) = sqlite_pool {
//         Ok(pool)
//     } else {
//         let pool = SqlitePool::connect(DB_URL).await?;
//         Ok(&pool)
//     }
// }
