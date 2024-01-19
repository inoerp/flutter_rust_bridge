use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct RikdataApplication {
  pub id : Option<i32> ,
  pub name : Option<String>,
  pub code : Option<String>,
  pub description : Option<String>,
  #[serde(rename = "configFilePath")]
 pub config_file_path  : Option<String>,
  pub r#type : Option<String>,
  pub status : Option<String>,
  #[serde(rename = "isInstalled")]
 pub is_installed  : Option<i32> ,
  #[serde(rename = "actionHistory")]
 pub action_history  : Option<String>,
  pub version : Option<String>,
  #[serde(rename = "imagePath")]
 pub image_path  : Option<String>,
  pub icon : Option<String>,
  #[serde(rename = "basePath")]
 pub base_path  : Option<String>,
  #[serde(rename = "dbType")]
 pub db_type  : Option<String>,
  #[serde(rename = "dbConnName")]
 pub db_conn_name  : Option<String>,
  #[serde(rename = "applicationType")]
 pub application_type  : Option<String>,
  #[serde(rename = "searchResultKey")]
 pub search_result_key  : Option<String>,
}

impl RikdataApplication {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<RikdataApplication>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<RikdataApplication> = sqlx::query_as::<_, RikdataApplication>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<RikdataApplication> = sqlx::query_as::<_, RikdataApplication>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<RikdataApplication>, Box<dyn Error>> {
            let sql: &str = "SELECT * from rikdata_application ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<RikdataApplication>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from rikdata_application WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<RikdataApplication>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from rikdata_application WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, RikdataApplication>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<RikdataApplication> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<RikdataApplication> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = RikdataApplication::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = RikdataApplication::find_by_id(Option::None,"1").await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_params() {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("type", "oracleCloudSimilar");
    let data = RikdataApplication::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


