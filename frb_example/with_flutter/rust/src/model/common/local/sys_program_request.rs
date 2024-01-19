use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct SysProgramRequest {
  #[serde(rename = "sysProgramRequestId")]
 pub sys_program_request_id  : Option<i32> ,
  #[serde(rename = "sysProgramHeaderId")]
 pub sys_program_header_id  : Option<i32> ,
  #[serde(rename = "requestParameters")]
 pub request_parameters  : Option<String>,
  #[serde(rename = "frequencyUom")]
 pub frequency_uom  : Option<String>,
  #[serde(rename = "frequencyValue")]
 pub frequency_value  : Option<i32> ,
  #[serde(rename = "applicationCode")]
 pub application_code  : Option<String>,
  #[serde(rename = "nextStartTime")]
 pub next_start_time  : Option<String>,
  #[serde(rename = "plannedStartTime")]
 pub planned_start_time  : Option<String>,
  #[serde(rename = "plannedEndTime")]
 pub planned_end_time  : Option<String>,
  #[serde(rename = "actualStartTime")]
 pub actual_start_time  : Option<String>,
  #[serde(rename = "completionTime")]
 pub completion_time  : Option<String>,
  #[serde(rename = "scheduleStartTime")]
 pub schedule_start_time  : Option<String>,
  #[serde(rename = "scheduleEndTime")]
 pub schedule_end_time  : Option<String>,
  #[serde(rename = "requestStatus")]
 pub request_status  : Option<String>,
  #[serde(rename = "logFilePath")]
 pub log_file_path  : Option<String>,
  #[serde(rename = "jsFilePath")]
 pub js_file_path  : Option<String>,
  #[serde(rename = "outputFilePath")]
 pub output_file_path  : Option<String>,
  #[serde(rename = "jsFunctionName")]
 pub js_function_name  : Option<String>,
  #[serde(rename = "requestType")]
 pub request_type  : Option<String>,
  #[serde(rename = "requestMessage")]
 pub request_message  : Option<String>,
  #[serde(rename = "parentRequestId")]
 pub parent_request_id  : Option<i32> ,
  #[serde(rename = "requestComment")]
 pub request_comment  : Option<String>,
  #[serde(rename = "sysProgramScheduleId")]
 pub sys_program_schedule_id  : Option<i32> ,
  #[serde(rename = "notifGroupId")]
 pub notif_group_id  : Option<i32> ,
  #[serde(rename = "notifTemplateId")]
 pub notif_template_id  : Option<i32> ,
  #[serde(rename = "createdBy")]
 pub created_by  : Option<String>,
  #[serde(rename = "creationDate")]
 pub creation_date  : Option<String>,
  #[serde(rename = "lastUpdateDate")]
 pub last_update_date  : Option<String>,
  #[serde(rename = "lastUpdatedBy")]
 pub last_updated_by  : Option<String>,
}

impl SysProgramRequest {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<SysProgramRequest>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<SysProgramRequest> = sqlx::query_as::<_, SysProgramRequest>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<SysProgramRequest> = sqlx::query_as::<_, SysProgramRequest>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<SysProgramRequest>, Box<dyn Error>> {
            let sql: &str = "SELECT * from sys_program_request ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<SysProgramRequest>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from sys_program_request WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<SysProgramRequest>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from sys_program_request WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, SysProgramRequest>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<SysProgramRequest> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<SysProgramRequest> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = SysProgramRequest::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = SysProgramRequest::find_by_id(Option::None,"1").await;
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
    let data = SysProgramRequest::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


