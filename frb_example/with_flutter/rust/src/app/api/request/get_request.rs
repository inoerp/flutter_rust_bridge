use crate::app::cache::global_cache::{self};
use crate::app::js::entity::js_trigger_point::JsTriggerPoint;
use crate::app::js::validation::JsValidation;
use crate::app::system::error::no_value::NoValueFoundError;

use crate::db::query::QueryData;
use crate::db::select::Select;
use crate::model::common::adv::adv_menu_path::AdvMenuPath;

use crate::model::state::global_state::GlobalState;


use sqlx::Pool;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GetRequest<'a> {
    pub gs: &'a GlobalState,
    pub qd: &'a QueryData,
    pub sqlite_pool: Option<&'a Pool<sqlx::Sqlite>>,
}

impl<'a> GetRequest<'a> {
    pub fn new(gs: &'a GlobalState, qd: &'a QueryData) -> Result<Self, Box<dyn Error>> {
        let sqlite_pool: Option<&Pool<sqlx::Sqlite>> = gs.sqlite_pools.get("local");
        Ok(Self {
            gs,
            qd,
            sqlite_pool,
        })
    }

    pub async fn complete_request(&self) -> Result<String, NoValueFoundError> {
        let application_code = self
            .qd
            .request_data
            .application
            .code
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("application_code"))?;
        let menu: AdvMenuPath = global_cache::get_menu(
            self.qd.request_data.base_path.as_ref(),
            application_code,
            self.qd.request_data.entity_path.as_ref(),
            self.sqlite_pool,
        )
        .await?;
        // let menu = AdvMenuPath::new(
        //     application_code.to_string(),
        //     self.qd.request_data.entity_path.to_string(),
        //     self.sqlite_pool,
        // )
        // .await?;
        if let Some(conn_mapping) = self
            .gs
            .conn_pools
            .lock()
            .await
            .get(&self.qd.request_data.base_path)
        {
            let db_data: Vec<linked_hash_map::LinkedHashMap<String, serde_json::Value>> =
                Select::new(conn_mapping)
                    .fetch_for_get_request(self.qd, &menu)
                    .await?;
            //complete js validation after get
            let js_validation = JsValidation::new(
                &self.qd.request_data.base_path,
                &self.qd.request_data.entity_path,
            );
            let val_result = js_validation
                .validate_after(JsTriggerPoint::AfterGet, &self.qd.request_data, &db_data)
                .await?;
            if !val_result.rd_proceed_status {
                return Ok(val_result.rd_proceed_message);
            }
            let ret_str = serde_json::json!(db_data).to_string();
            Ok(ret_str)
        } else {
            Ok("Invalid database connection".to_string())
        }
    }
}
