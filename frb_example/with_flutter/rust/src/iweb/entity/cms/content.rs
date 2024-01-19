use crate::db::model::db_conn_map::DbPool;
use crate::{
    app::system::error::no_value::NoValueFoundError, db::model::db_conn_map::DbConnMapping,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct Content {
    pub content_id: u32,
    pub category_id: u32,
    pub user_name: String,
    //pub created_on: String,
    pub vid: u32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub teaser: String,
    pub log: String,
    pub timestamp: i32,
    pub format: i32,
}

impl Content {
    pub fn no_content() -> Self {
        Self {
            content_id: 0,
            category_id:0,
            user_name: "".to_string(),
            vid: 0,
            user_id:0,
            title: "No Content".to_string(),
            body: "No document was found. Use the back button to go to the previous page."
            .to_string(),
            teaser: "No teaser was found. Use the back button to go to the previous page."
            .to_string(),
            log: "No Content".to_string(),
            timestamp: 0,
            format: 0,
        }
    }

    pub async fn find_all(
        conn_mapping: &DbConnMapping,
        category_id: &str,
    ) -> Result<Vec<Content>, NoValueFoundError> {
        let sql = format!(
            "SELECT * from content_details_v where category_id='{}'",
            category_id
        );

        if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
            let mut conn = conn_pool.acquire().await.map_err(|err| {
                NoValueFoundError::new(
                    format!("Unable to get db conn from pool. Err {:?}", err).as_str(),
                )
            })?;
            let records: Vec<Content> = sqlx::query_as::<_, Content>(sql.as_str())
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
