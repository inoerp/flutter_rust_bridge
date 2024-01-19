use crate::db::isqlite::DB_URL;
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct ActionAssignment {
    pub id: Option<i32>,
    #[serde(rename = "actionHeaderId")]
    pub action_header_id: Option<i32>,
    #[serde(rename = "menuPathCode")]
    pub menu_path_code: Option<String>,
    #[serde(rename = "menuPathId")]
    pub menu_path_id: Option<i32>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

impl ActionAssignment {
    pub async fn find_by_sql(
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        sql: &str,
    ) -> Result<Vec<ActionAssignment>, Box<dyn Error>> {
        if let Some(pool) = sqlite_pool {
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<ActionAssignment> = sqlx::query_as::<_, ActionAssignment>(sql)
                .fetch_all(&mut connection)
                .await?;
            Ok(data_rows)
        } else {
            let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<ActionAssignment> = sqlx::query_as::<_, ActionAssignment>(sql)
                .fetch_all(&mut connection)
                .await?;
            Ok(data_rows)
        }
    }

    pub async fn find_all(
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
    ) -> Result<Vec<ActionAssignment>, Box<dyn Error>> {
        let sql: &str = "SELECT * from action_assignment ";
        Self::find_by_sql(sqlite_pool, sql).await
    }

    pub async fn find_by_id(
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        id: &str,
    ) -> Result<Vec<ActionAssignment>, Box<dyn Error>> {
        let sql1: String = format!("SELECT * from action_assignment WHERE id = '{}' ", id);
        Self::find_by_sql(sqlite_pool, sql1.as_str()).await
    }

    pub async fn find_by_params(
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<ActionAssignment>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from action_assignment WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
        if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, ActionAssignment>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<ActionAssignment> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<ActionAssignment> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_all() {
        let data = ActionAssignment::find_all(Option::None).await;
        match data {
            Ok(d) => {
                log::info!("found data. length {:?}", d.len());
            }
            Err(err) => log::info!("failed to find data , error {:?} ", err),
        }
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let data = ActionAssignment::find_by_id(Option::None, "1").await;
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
        let data = ActionAssignment::find_by_params(Option::None, params).await;
        match data {
            Ok(d) => {
                log::info!("found data. length {:?}", d);
            }
            Err(err) => log::info!("failed to find data , error {:?} ", err),
        }
    }
}
