pub mod db_actions;
pub mod execute;
pub mod select;

use sqlx::Pool;
use std::collections::HashMap;
use std::error::Error;

use super::super::configuration::Settings;
use super::model::db_conn_map::{DbConnMapping, DbPool};
use super::model::db_type::DbType;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::configuration;
use crate::model::common::local::rikdata_application::RikdataApplication;

pub struct IMySql;

impl IMySql {
    pub async fn get_connection_pools(
        settings: &Settings,
        apps: &Vec<RikdataApplication>,
    ) -> Result<HashMap<String, Pool<sqlx::MySql>>, Box<dyn Error>> {
        let mut mysql_pools: HashMap<String, Pool<sqlx::MySql>> = HashMap::new();
        for conn in &settings.db_settings {
            if conn.db_type.eq_ignore_ascii_case("MySql") {
                let conn_str = conn.connection_string_mysql();
                let connection_pool = sqlx::MySqlPool::connect(&conn_str).await?;
                let mut key: Option<String> = None;
                for app in apps {
                    let conn_name = app.db_conn_name.as_ref();
                    if let Some(conn_name) = conn_name {
                        if conn_name.eq_ignore_ascii_case(&conn.name) {
                            let base_path = app
                                .base_path
                                .as_ref()
                                .ok_or_else(|| NoValueFoundError::new("Unable to fetch base path"))?
                                .to_string();
                            key = Some(base_path);
                        }
                    }
                }
                if let Some(val) = key {
                    mysql_pools.insert(val, connection_pool);
                }
            }
        }
        Ok(mysql_pools)
    }

    pub async fn get_connection_pools_for_test(
    ) -> Result<HashMap<String, Pool<sqlx::MySql>>, Box<dyn Error>> {
        let config = configuration::get_configuration()?;
        let all_applications: Vec<RikdataApplication> =
            RikdataApplication::find_all(Option::None).await?;
        let mysql_pools = Self::get_connection_pools(&config, &all_applications).await?;
        Ok(mysql_pools)
    }

    pub async fn get_conn_mapping_test(
    ) -> Result<DbConnMapping, Box<dyn Error>> {
        let config = configuration::get_configuration()?;
        let all_applications: Vec<RikdataApplication> =
            RikdataApplication::find_all(Option::None).await?;
        let mysql_pools = Self::get_connection_pools(&config, &all_applications).await?;
        let db_pool = DbPool::MySql(mysql_pools.get("ierp").expect("Unable to find pool for ierp").to_owned());
        let map: DbConnMapping = DbConnMapping::new("ierp".to_string(), db_pool, DbType::MySql);
        Ok(map)
    }
}
