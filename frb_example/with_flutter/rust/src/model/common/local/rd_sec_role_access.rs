use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct RdSecRoleAccess {
  pub id : Option<i32> ,
  #[serde(rename = "accessLevel")]
 pub access_level  : Option<i32> ,
  #[serde(rename = "createdBy")]
 pub created_by  : Option<String>,
  #[serde(rename = "creationDate")]
 pub creation_date  : Option<String>,
  #[serde(rename = "lastUpdateDate")]
 pub last_update_date  : Option<String>,
  #[serde(rename = "lastUpdatedBy")]
 pub last_updated_by  : Option<String>,
  #[serde(rename = "objClassName")]
 pub obj_class_name  : Option<String>,
  #[serde(rename = "objName")]
 pub obj_name  : Option<String>,
  #[serde(rename = "roleCode")]
 pub role_code  : Option<String>,
  #[serde(rename = "secRoleId")]
 pub sec_role_id  : Option<i32> ,
}

impl RdSecRoleAccess {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<RdSecRoleAccess>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<RdSecRoleAccess> = sqlx::query_as::<_, RdSecRoleAccess>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<RdSecRoleAccess> = sqlx::query_as::<_, RdSecRoleAccess>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<RdSecRoleAccess>, Box<dyn Error>> {
            let sql: &str = "SELECT * from rd_sec_role_access ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<RdSecRoleAccess>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from rd_sec_role_access WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<RdSecRoleAccess>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from rd_sec_role_access WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, RdSecRoleAccess>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<RdSecRoleAccess> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<RdSecRoleAccess> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = RdSecRoleAccess::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = RdSecRoleAccess::find_by_id(Option::None,"1").await;
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
    let data = RdSecRoleAccess::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


