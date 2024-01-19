use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct AppInstance {
  pub id : Option<i32> ,
  pub r#type : Option<String>,
  #[serde(rename = "instanceName")]
 pub instance_name  : Option<String>,
  #[serde(rename = "instanceCode")]
 pub instance_code  : Option<String>,
  #[serde(rename = "restTypeCode")]
 pub rest_type_code  : Option<String>,
  pub sequence : Option<i32> ,
  pub icon : Option<String>,
  pub description : Option<String>,
  #[serde(rename = "hostName")]
 pub host_name  : Option<String>,
  #[serde(rename = "baseApi")]
 pub base_api  : Option<String>,
  #[serde(rename = "authorizationType")]
 pub authorization_type  : Option<String>,
  #[serde(rename = "userName")]
 pub user_name  : Option<String>,
  pub password : Option<String>,
  #[serde(rename = "loginKey")]
 pub login_key  : Option<String>,
  #[serde(rename = "loginUrl")]
 pub login_url  : Option<String>,
  #[serde(rename = "authorizationEndPoint")]
 pub authorization_end_point  : Option<String>,
  #[serde(rename = "tokenEndPoint")]
 pub token_end_point  : Option<String>,
  pub identifier : Option<String>,
  pub secret : Option<String>,
  #[serde(rename = "hostPort")]
 pub host_port  : Option<i32> ,
  #[serde(rename = "redirectUrl")]
 pub redirect_url  : Option<String>,
  #[serde(rename = "applicationCode")]
 pub application_code  : Option<String>,
  #[serde(rename = "dashboardName")]
 pub dashboard_name  : Option<String>,
  #[serde(rename = "authScopes")]
 pub auth_scopes  : Option<String>,
  #[serde(rename = "desktopRedirectUrl")]
 pub desktop_redirect_url  : Option<String>,
  #[serde(rename = "storageType")]
 pub storage_type  : Option<String>,
}

impl AppInstance {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<AppInstance>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<AppInstance> = sqlx::query_as::<_, AppInstance>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<AppInstance> = sqlx::query_as::<_, AppInstance>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<AppInstance>, Box<dyn Error>> {
            let sql: &str = "SELECT * from app_instance ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<AppInstance>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from app_instance WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<AppInstance>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from app_instance WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, AppInstance>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<AppInstance> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<AppInstance> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = AppInstance::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = AppInstance::find_by_id(Option::None,"1").await;
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
    let data = AppInstance::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


