use std::sync::{Arc, Mutex};

use crate::db::model::db_conn_map::DbPool;
use crate::{
    app::system::error::no_value::NoValueFoundError, db::model::db_conn_map::DbConnMapping,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

lazy_static! {
    static ref GC_SITE_INFO: Arc<Mutex<SiteInfo>> = Arc::new(Mutex::new(SiteInfo::no_content()));
    //static ref GC_SITE_INIT: Once = Once::new(); //TODO use Once
}

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct SiteInfo {
    pub site_name: String,
    pub description1: String,
    pub description2: String,
    pub contact_message: String,
    pub youtube_link: String,
    pub video_link: String,
    pub twitter_link: String,
}

impl SiteInfo {
    pub fn no_content() -> Self {
        Self {
            site_name: "WebSite".to_string(),
            description1: "Site Description1".to_string(),
            description2: "Default Site Description 2".to_string(),
            contact_message: "contact : nomail@WebSite.com".to_string(),
            youtube_link: "#".to_string(),
            video_link: "#".to_string(),
            twitter_link: "#".to_string(),
        }
    }

    pub async fn get_site_info(
        conn_mapping: &DbConnMapping,
    ) -> Result<SiteInfo, NoValueFoundError> {
        let mut site_info = GC_SITE_INFO
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to get site info"))?
            .clone();
        if site_info.site_name.eq_ignore_ascii_case("website") {
            let site_infos = Self::find_all(conn_mapping).await?;
            if !site_infos.is_empty() {
                if let Some(val) = site_infos.first() {
                    site_info = val.clone();
                }
            }
        }
        Ok(site_info)
    }

    async fn find_all(conn_mapping: &DbConnMapping) -> Result<Vec<SiteInfo>, NoValueFoundError> {
        let sql = "SELECT * FROM site_info LIMIT 1".to_string();

        if let DbPool::MySql(conn_pool) = &conn_mapping.conn_pool {
            let mut conn = conn_pool.acquire().await.map_err(|err| {
                NoValueFoundError::new(
                    format!("Unable to get db conn from pool. Err {:?}", err).as_str(),
                )
            })?;
            let records: Vec<SiteInfo> = sqlx::query_as::<_, SiteInfo>(sql.as_str())
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
