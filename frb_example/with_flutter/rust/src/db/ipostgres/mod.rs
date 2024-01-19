pub mod select;

use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use std::collections::HashMap;
use std::error::Error;

use super::super::configuration::Settings;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::model::common::local::rikdata_application::RikdataApplication;

pub struct IPostgres;

impl IPostgres {
    pub async fn get_connection_pools(
        settings: &Settings,
        apps: &Vec<RikdataApplication>,
    ) -> Result<HashMap<String, Pool<sqlx::Postgres>>, Box<dyn Error>> {
        let mut pools: HashMap<String, Pool<sqlx::Postgres>> = HashMap::new();
        for conn in &settings.db_settings {
            if conn.db_type.eq_ignore_ascii_case("Postgres") {
                let conn_str = conn.connection_string_mysql();
                let connection_pool = PgPoolOptions::new()
                    .max_connections(10)
                    .connect(&conn_str)
                    .await?;
                let mut key: Option<String> = None;
                for app in apps {
                    let conn_name = app.db_conn_name.as_ref();
                    if let Some(conn_name) = conn_name {
                        if conn_name.eq_ignore_ascii_case(&conn.name) {
                            let base_path = app
                                .base_path
                                .as_ref()
                                .ok_or_else(|| NoValueFoundError::new("Unable to find base path"))?
                                .to_string();
                            key = Some(base_path);
                        }
                    }
                }
                if let Some(val) = key {
                    pools.insert(val, connection_pool);
                }
            }
        }
        Ok(pools)
    }
}
