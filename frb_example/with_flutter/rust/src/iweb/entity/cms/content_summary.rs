use crate::db::model::db_conn_map::DbPool;
use crate::{
    app::system::error::no_value::NoValueFoundError, db::model::db_conn_map::DbConnMapping,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct ContentSummary {
    pub no_of_articles: i32,
    pub category_id: u32,
    pub category_name: String,
    pub category_group: String,
}

impl ContentSummary {
    pub fn no_content() -> Self {
        Self {
            no_of_articles: 0,
            category_id: 0,
            category_name: "No document was found. Use the back button to go to the previous page."
                .to_string(),
            category_group: "No Content".to_string(),
        }
    }

    pub async fn find_all(
        conn_mapping: &DbConnMapping,
    ) -> Result<Vec<ContentSummary>, NoValueFoundError> {
        let sql = "SELECT * FROM content_summary_v order by no_of_articles desc;";

        if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
            let mut conn = conn_pool.acquire().await.map_err(|err| {
                NoValueFoundError::new(
                    format!("Unable to get db conn from pool. Err {:?}", err).as_str(),
                )
            })?;
            let records: Vec<ContentSummary> = sqlx::query_as::<_, ContentSummary>(sql)
                .fetch_all(&mut conn)
                .await
                .map_err(|err| {
                    NoValueFoundError::new(
                        format!("Unable to find any records. Err {:?}", err).as_str(),
                    )
                })?;
            Ok(records)
        } else {
            Err(NoValueFoundError::new("Unable to get mysql db pool"))
        }
    }
}
