use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct SysProgramSchedule {
  #[serde(rename = "sysProgramScheduleId")]
 pub sys_program_schedule_id  : Option<i32> ,
  #[serde(rename = "scheduleName")]
 pub schedule_name  : Option<String>,
  #[serde(rename = "frequencyUom")]
 pub frequency_uom  : Option<String>,
  #[serde(rename = "frequencyValue")]
 pub frequency_value  : Option<i32> ,
  #[serde(rename = "increaseDateParameterCb")]
 pub increase_date_parameter_cb  : Option<i32> ,
  #[serde(rename = "scheduleDescription")]
 pub schedule_description  : Option<String>,
  #[serde(rename = "scheduleStartTime")]
 pub schedule_start_time  : Option<String>,
  #[serde(rename = "scheduleEndTime")]
 pub schedule_end_time  : Option<String>,
  #[serde(rename = "createdBy")]
 pub created_by  : Option<String>,
  #[serde(rename = "creationDate")]
 pub creation_date  : Option<String>,
  #[serde(rename = "lastUpdateDate")]
 pub last_update_date  : Option<String>,
  #[serde(rename = "lastUpdatedBy")]
 pub last_updated_by  : Option<String>,
}

impl SysProgramSchedule {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<SysProgramSchedule>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<SysProgramSchedule> = sqlx::query_as::<_, SysProgramSchedule>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<SysProgramSchedule> = sqlx::query_as::<_, SysProgramSchedule>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<SysProgramSchedule>, Box<dyn Error>> {
            let sql: &str = "SELECT * from sys_program_schedule ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<SysProgramSchedule>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from sys_program_schedule WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<SysProgramSchedule>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from sys_program_schedule WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, SysProgramSchedule>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<SysProgramSchedule> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<SysProgramSchedule> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = SysProgramSchedule::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = SysProgramSchedule::find_by_id(Option::None,"1").await;
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
    let data = SysProgramSchedule::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


