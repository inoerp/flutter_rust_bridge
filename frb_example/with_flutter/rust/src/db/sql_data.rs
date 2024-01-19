use crate::{
    app::api::request::model::request_data::{ RequestDataSimple},
    app::{cache::global_cache::GlobalCache, system::error::no_value::NoValueFoundError},
};

use crate::db::model::db_conn_map::DbPool;
use crate::db::model::db_type::DbType;
use super::imysql::IMySql;



#[derive(Debug, Clone, PartialEq)]
pub enum SqlActionType {
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
pub struct SqlData<'a> {
    pub sql: &'a str,
    pub params: Vec<String>, //sql parameters
    pub action_type: SqlActionType,
    request_data: RequestDataSimple,
}

impl<'a> SqlData<'a> {
    pub fn new(
        sql: &'a str,
        params: Vec<String>,
        action_type: SqlActionType,
        request_data: RequestDataSimple,
    ) -> Self {
        Self {
            sql,
            params,
            action_type,
            request_data,
        }
    }

    pub async fn complete_simple_request_mysql(
            base_path: &str,
            sql: &str,
            params: Vec<String>,
        ) -> Result<Vec<linked_hash_map::LinkedHashMap<String, String>>, NoValueFoundError> {
            let gs = GlobalCache::get_global_state()?;
            let polls1 = gs.conn_pools.lock().await;
            if let Some(conn_mapping) = polls1.get(base_path) {
                match conn_mapping.db_type {
                    DbType::MySql => {
                        if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
                            //let pool = conn_pool.lock().await.to_owned();
                            let json_data: Vec<linked_hash_map::LinkedHashMap<String, String>> =
                            IMySql::select_using_sql(conn_pool, sql, &params, ).await?;
                            //let ret_str = serde_json::json!(&json_data).to_string();
                            Ok(json_data)
                        } else {
                            Err(NoValueFoundError::new("No db connection found for MySQL"))
                        }
                    }
                    // DbType::Postgres => {
                    //     if let DbPool::Postgres(conn_pool) = &conn_mapping.conn_pool {
                    //         let ret_str = ipostgres::select::get_data_from_db(conn_pool, self.qd).await;
                    //         return Ok(ret_str);
                    //     } else {
                    //         return Ok("Invalid database connection".to_string());
                    //     }
                    // }
                    // DbType::Sqlite => {
                    //     if let DbPool::Sqlite(conn_pool) = &conn_mapping.conn_pool {
                    //         let ret_str = isqlite::select::get_data_from_db(conn_pool, self.qd).await;
                    //         return Ok(ret_str);
                    //     } else {
                    //         return Ok("Invalid database connection for sqlite db".to_string());
                    //     }
                    // }
                    // crate::db::model::db_type::DbType::Sqlite => {},
                    // crate::db::model::db_type::DbType::MsSql => {},
                    // crate::db::model::db_type::DbType::Oracle => {},
                    _ => {
                        Err(NoValueFoundError::new("No mapping found for dbtype"))
                    }
                }
            } else {
                Err(NoValueFoundError::new("No db mapping found for base path"))
            }
        }
    
     

    pub async fn complete_request(&self) -> Result<String, NoValueFoundError> {
        let gs = GlobalCache::get_global_state()?;
        let pools1 = gs.conn_pools.lock().await;
        if let Some(conn_mapping) = pools1.get(&self.request_data.base_path) {
            match conn_mapping.db_type {
                DbType::MySql => {
                    if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
                        //let pool = conn_pool.lock().await.to_owned();
                        let json_data = IMySql::select_using_sql(
                            conn_pool,
                            self.sql,
                            &self.params,
                        )
                        .await?;
                        let ret_str = serde_json::json!(&json_data).to_string();
                        Ok(ret_str)
                    } else {
                        Ok("Invalid database connection".to_string())
                    }
                }
                // DbType::Postgres => {
                //     if let DbPool::Postgres(conn_pool) = &conn_mapping.conn_pool {
                //         let ret_str = ipostgres::select::get_data_from_db(conn_pool, self.qd).await;
                //         return Ok(ret_str);
                //     } else {
                //         return Ok("Invalid database connection".to_string());
                //     }
                // }
                // DbType::Sqlite => {
                //     if let DbPool::Sqlite(conn_pool) = &conn_mapping.conn_pool {
                //         let ret_str = isqlite::select::get_data_from_db(conn_pool, self.qd).await;
                //         return Ok(ret_str);
                //     } else {
                //         return Ok("Invalid database connection for sqlite db".to_string());
                //     }
                // }
                // crate::db::model::db_type::DbType::Sqlite => {},
                // crate::db::model::db_type::DbType::MsSql => {},
                // crate::db::model::db_type::DbType::Oracle => {},
                _ => {
                    Ok("Invalid database connection".to_string())
                }
            }
        } else {
            Ok("Invalid database connection".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::api::request::model::request_data::RequestDataSimple;
    use crate::startup;

    #[tokio::test]
    async fn test_select() {
        let res = startup::init().await;
        if let Err(err) = res{
            log::error!("Error in startup {:?}", err);
        }
        let sql = "SELECT * from gl_payment_term";
        let params: Vec<String> = Vec::new();
        let request_data = RequestDataSimple {
            base_path: "ierp".to_string(),
            entity_path: "GlPaymentTerm".to_string(),
            action_path: "".to_string(),
            entity_table: "gl_payment_term".to_string(),
            entity_base_table: "gl_payment_term".to_string(),
            application_code: "Inoerp".to_string(),
        };
        let sql_data: SqlData = SqlData::new(sql, params, SqlActionType::Select, request_data);
        let _data = sql_data.complete_request().await;

    }

    #[tokio::test]
    async fn test_select_simple() {
        let res = startup::init().await;
        if let Err(err) = res{
            log::error!("Error in startup {:?}", err);
        }
        let sql = "SELECT * from gl_payment_term";
        let params: Vec<String> = Vec::new();
        let _data = SqlData::complete_simple_request_mysql("ierp", sql, params).await;

    }
}
