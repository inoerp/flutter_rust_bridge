use std::collections::HashMap;

use crate::app::cache::global_cache::GlobalCache;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::imysql::IMySql;
use crate::db::ipostgres::IPostgres;
use crate::db::model::db_conn_map::DbPool;
use crate::db::model::db_type::DbType;
use crate::db::sql_data::SqlActionType;
use crate::db::{imysql};
use crate::model::state::global_state::GlobalState;

use sqlx::{MySql, Pool, Postgres};

pub struct JsDbAction {
    action_type: SqlActionType,
    arg: HashMap<String, String>,
}

impl JsDbAction {
    pub fn new(action_type: SqlActionType, arg: HashMap<String, String>) -> Self {
        Self { action_type, arg }
    }

    pub async fn complete_task(&self) -> Result<String, NoValueFoundError> {
        match self.action_type {
            SqlActionType::Select => self.select_task().await,
            SqlActionType::Update | SqlActionType::Delete | SqlActionType::Insert => {
                self.update_task().await
            }
        }
    }

    async fn update_task(&self) -> Result<String, NoValueFoundError> {
        let (sql, db_type, gs, base_path) = self.get_arg_values()?;

        let ret_str = match db_type {
            DbType::MySql => {
                let pool: Pool<MySql> = Self::get_mysql_pool(&gs, &base_path).await?;
                let params: Vec<String> = Vec::new();
                let json_data = imysql::execute::db_execution_simple(
                    &pool,
                    sql,
                    &params,
                    self.action_type.clone(),
                )
                .await?;
                serde_json::json!(&json_data).to_string()
              }
            DbType::Postgres => todo!(),
            DbType::Sqlite => todo!(),
            DbType::MsSql => todo!(),
            DbType::Oracle => todo!(),
        };
        Ok(ret_str)
    }

    async fn select_task(&self) -> Result<String, NoValueFoundError> {
        let (sql, db_type, gs, base_path) = self.get_arg_values()?;

        let ret_str = match db_type {
            DbType::MySql => {
                let pool: Pool<MySql> = Self::get_mysql_pool(&gs, &base_path).await?;
                let json_data = IMySql::select_using_sql(&pool,sql, &vec![] ).await?;
                serde_json::json!(&json_data).to_string()
                
            }
            DbType::Postgres => {
                let pool: Pool<Postgres> = Self::get_postgres_pool(&gs, &base_path).await?;
                let json_data = IPostgres::select_using_sql(&pool, sql, &vec![], ).await?;
                serde_json::json!(&json_data).to_string()
                
            }
            DbType::Sqlite => todo!(),
            DbType::MsSql => todo!(),
            DbType::Oracle => todo!(),
        };
        Ok(ret_str)
    }

    fn get_arg_values(&self) -> Result<(&String, DbType, GlobalState, String), NoValueFoundError> {
        let sql = self
            .arg
            .get("sql")
            .ok_or_else(|| NoValueFoundError::new("Sql is missing in JS request"))?;
        let db_type_str = self
            .arg
            .get("dbType")
            .ok_or_else(|| NoValueFoundError::new("dbType is missing in JS request"))?;
        let db_type = DbType::from_string(db_type_str);
        let conn_name_str = self
            .arg
            .get("connName")
            .ok_or_else(|| NoValueFoundError::new("connName is missing in JS request"))?;
        let gs: GlobalState = GlobalCache::get_global_state()?;
        let base_path = GlobalCache::get_base_path(conn_name_str)?;
        Ok((sql, db_type, gs, base_path))
    }

    async fn get_mysql_pool(
        gs: &GlobalState,
        base_path: &str,
    ) -> Result<Pool<MySql>, NoValueFoundError> {
        let pools1 = gs.conn_pools.lock().await;
        if let Some(conn_mapping) = pools1.get(base_path) {
            match conn_mapping.db_type {
                DbType::MySql => {
                    if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
                       // let pool = conn_pool.lock().await.to_owned();
                        Ok(conn_pool.to_owned())
                    } else {
                        Err(NoValueFoundError::new("No pool found for MySql"))
                    }
                }
                _ => {
                    Err(NoValueFoundError::new("No connection map found for MySQL"))
                }
            }
        } else {
            Err(NoValueFoundError::new(
                "No connection map found for basePath",
            ))
        }
    }

    async fn get_postgres_pool(
        gs: &GlobalState,
        base_path: &str,
    ) -> Result<Pool<Postgres>, NoValueFoundError> {
        let pools1 = gs.conn_pools.lock().await;
        if let Some(conn_mapping) = pools1.get(base_path) {
            match conn_mapping.db_type {
                DbType::Postgres => {
                    if let DbPool::Postgres(conn_pool) = &conn_mapping.conn_pool {
                        // let pool = conn_pool.lock().await.to_owned();
                        Ok(conn_pool.to_owned())
                    } else {
                        Err(NoValueFoundError::new("No pool found for MySql"))
                    }
                }
                _ => {
                    Err(NoValueFoundError::new("No connection map found for MySQL"))
                }
            }
        } else {
            Err(NoValueFoundError::new(
                "No connection map found for basePath",
            ))
        }
    }
}
