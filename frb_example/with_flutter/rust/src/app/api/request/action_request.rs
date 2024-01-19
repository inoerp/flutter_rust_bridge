use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::action::ActionData;
use crate::model::common::local::rikdata_application::RikdataApplication;
use crate::model::state::global_state::GlobalState;
use sqlx::Pool;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ActionRequest<'a> {
    pub gs: &'a GlobalState,
    pub ad: &'a ActionData,
    pub pri_key_val_map: HashMap<String, String>,
    pub app: &'a RikdataApplication,
    pub sqlite_pool: Option<&'a Pool<sqlx::Sqlite>>,
}

impl<'a> ActionRequest<'a> {
    pub fn new(gs: &'a GlobalState, ad: &'a ActionData) -> Result<Self, NoValueFoundError> {
        let pri_key_val_map: HashMap<String, String> = HashMap::new();
        let request_data = if let Some(request_data) = &ad.request_data {
            request_data
        } else {
            return Err(NoValueFoundError::new(
                "Base path is missing for the request",
            ));
        };
        let app: &RikdataApplication =
            gs.get_app_for_base_path(&request_data.base_path)
                .map_err(|err| {
                    NoValueFoundError::new(
                        format!("Unable to fetch rikdata app. Error {:?}", err).as_str(),
                    )
                })?;
        let sqlite_pool: Option<&Pool<sqlx::Sqlite>> = gs.sqlite_pools.get("local");
        Ok(Self {
            gs,
            ad,
            pri_key_val_map,
            app,
            sqlite_pool,
        })
    }
}
