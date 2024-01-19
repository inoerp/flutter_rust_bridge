use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct SavedSearchResult {
  pub id : Option<i32> ,
  #[serde(rename = "savedResultName")]
 pub saved_result_name  : Option<String>,
  #[serde(rename = "savedResultCode")]
 pub saved_result_code  : Option<String>,
  pub icon : Option<String>,
  pub description : Option<String>,
  #[serde(rename = "searchUrl")]
 pub search_url  : Option<String>,
  #[serde(rename = "queryLimit")]
 pub query_limit  : Option<i32> ,
  #[serde(rename = "queryOffset")]
 pub query_offset  : Option<i32> ,
  pub sequence : Option<i32> ,
  #[serde(rename = "defaultViewType")]
 pub default_view_type  : Option<String>,
  #[serde(rename = "menuPathId")]
 pub menu_path_id  : Option<i32> ,
  #[serde(rename = "resultItems")]
 pub result_items  : Option<String>,
  #[serde(rename = "appInstanceId")]
 pub app_instance_id  : Option<i32> ,
  #[serde(rename = "showInNotification")]
 pub show_in_notification  : Option<i32> ,
}

impl SavedSearchResult {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<SavedSearchResult>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<SavedSearchResult> = sqlx::query_as::<_, SavedSearchResult>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<SavedSearchResult> = sqlx::query_as::<_, SavedSearchResult>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<SavedSearchResult>, Box<dyn Error>> {
            let sql: &str = "SELECT * from saved_search_result ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<SavedSearchResult>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from saved_search_result WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<SavedSearchResult>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from saved_search_result WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, SavedSearchResult>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<SavedSearchResult> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<SavedSearchResult> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = SavedSearchResult::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = SavedSearchResult::find_by_id(Option::None,"1").await;
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
    let data = SavedSearchResult::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


