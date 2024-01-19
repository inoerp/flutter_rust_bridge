use crate::db::model::db_conn_map::DbPool;
use crate::{
    app::system::error::no_value::NoValueFoundError, db::model::db_conn_map::DbConnMapping,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub content_id: i32,
    pub comment_id: i32,
    pub content_created_by: String,
    //pub created_on: String,
    pub user_id: i32,
    pub title: String,
    pub subject: String,
    pub body: String,
    pub comment: String,
    pub name: String,
}

impl Comment {
    pub fn no_content() -> Self {
        Self {
            content_id: 0,
            comment_id: 0,
            content_created_by: "".to_string(),
            user_id: 0,
            title: "No Content".to_string(),
            subject: "Invalid document".to_string(),
            body: "No document was found. Use the back button to go to the previous page."
                .to_string(),
            comment: "No comment was found. Use the back button to go to the previous page."
                .to_string(),
            name: "UnKnown".to_string(),
        }
    }
    pub async fn find_all(
        conn_mapping: &DbConnMapping,
        content_id: &str,
    ) -> Result<Vec<Comment>, NoValueFoundError> {
        let sql = format!(
            "SELECT * from comment_details_v where content_id='{}'",
            content_id
        );

        if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
            let mut conn = conn_pool.acquire().await.map_err(|err| {
                NoValueFoundError::new(
                    format!("Unable to get db conn from pool. Err {:?}", err).as_str(),
                )
            })?;
            let records: Vec<Comment> = sqlx::query_as::<_, Comment>(sql.as_str())
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
