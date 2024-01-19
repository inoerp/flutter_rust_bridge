
use crate::app::system::error::no_value::NoValueFoundError;

use super::{
    action::ActionData,
    imysql,
    model::{
        db_conn_map::{DbConnMapping, DbPool},
        db_type::DbType,
    }, isqlite,
};
use linked_hash_map::LinkedHashMap;

pub struct DbExecution;

impl DbExecution {
    pub async fn execute(
        conn_mapping: &DbConnMapping,
        ad: &ActionData,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        match conn_mapping.db_type {
            DbType::MySql => {
                if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
                    let ret_message = imysql::execute::db_execution(conn_pool, ad).await?;
                    Ok(ret_message)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            DbType::Sqlite => {
                if let DbPool::Sqlite(conn_pool) = &conn_mapping.conn_pool {
                    let ret_message = isqlite::execute::db_execution(conn_pool, ad).await?;
                    Ok(ret_message)
                } else {
                    Err(NoValueFoundError::new("Invalid database connection"))
                }
            }
            // crate::db::model::db_type::DbType::Sqlite => {},
            // crate::db::model::db_type::DbType::MsSql => {},
            // crate::db::model::db_type::DbType::Oracle => {},
            _ => Err(NoValueFoundError::new("Invalid database connection")),
        }
    }
}
