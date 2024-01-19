
use sqlx::Pool;
use crate::db::model::db_type::DbType;

#[derive(Debug, Clone)]
pub enum DbPool {
    //MySql(Arc<Mutex<Pool<sqlx::MySql>>>),
    MySql(Pool<sqlx::MySql>),
    Postgres(Pool<sqlx::Postgres>),
    Sqlite(Pool<sqlx::Sqlite>)
}

#[derive(Debug, Clone)]
pub struct DbConnMapping{
    pub base_path: String,
    pub conn_pool: DbPool,
    pub db_type: DbType
}

impl DbConnMapping {
    pub fn new(base_path: String,
         conn_pool: DbPool,
         db_type: DbType) -> Self{
          Self { base_path, conn_pool, db_type }
    }
}