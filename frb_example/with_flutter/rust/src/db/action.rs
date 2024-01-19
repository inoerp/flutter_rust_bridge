use super::copy_action::child_copy::ParentChildData;
use super::db_delete::DbDelete;
use super::{insert, update};
use crate::app::api::request::model::request_data::RequestData;
use crate::app::cache::global_cache::{self};
use crate::app::sec::user_session::UserSessionData;
use crate::db::sql_data::SqlActionType;

use super::copy_action::copy::{self, DocCopyLevel};
use crate::model::common::local::rikdata_application::RikdataApplication;
use crate::model::entity::copy_action::CopyActionData;
use crate::model::state::global_state::GlobalState;
use crate::{
    app::api::request::model::body_data::BodyData, app::system::error::no_value::NoValueFoundError,
};

#[derive(Debug, Clone)]
pub struct ActionData {
    // gs: &'a GlobalState,
    pub sql: String,
    pub params: Vec<String>,
    pub request_data: Option<RequestData>,
    pub action_type: SqlActionType,
    //pub menu: Option<AdvMenuPath>
}

impl ActionData {
    pub async fn init(
        gs: &GlobalState,
        request_data: RequestData,
        action_type: SqlActionType,
        session_data: &UserSessionData,
    ) -> Result<Self, NoValueFoundError> {
        let (sql, params) = match action_type {
            SqlActionType::Insert => {
                Self::get_insert_sql(gs, &request_data, &request_data.body_data, session_data)
                    .await?
            }
            SqlActionType::Delete => Self::get_delete_sql(gs, &request_data, session_data).await?,
            _ => Self::get_update_sql(gs, &request_data, session_data).await?,
        };

        let ad = ActionData {
            sql,
            params,
            request_data: Some(request_data),
            action_type,
        };
        Ok(ad)
    }

    pub async fn init_for_copy(
        gs: &GlobalState,
        request_data: RequestData,
        action_type: SqlActionType,
        copy_action_data: &CopyActionData,
        session_data: &UserSessionData,
        copy_level: DocCopyLevel,
        parent_data: Option<&ParentChildData>,
    ) -> Result<Self, NoValueFoundError> {
        let (sql, params) = copy::copy_sql(
            gs,
            request_data.clone(),
            copy_action_data,
            session_data,
            copy_level,
            parent_data,
        )
        .await?;

        let query = sql;
        let ad = ActionData {
            sql: query,
            params,
            request_data: Some(request_data),
            action_type,
        };
        Ok(ad)
    }

    pub fn int_for_sql(
        action_type: SqlActionType,
        sql: String,
        params: Vec<String>,
    ) -> Self {
        ActionData {
            sql,
            params,
            request_data: None,
            action_type,
        }
    }


    pub fn new(
        request_data: RequestData,
        action_type: SqlActionType,
        sql: String,
        params: Vec<String>,
    ) -> Self {
        ActionData {
            sql,
            params,
            request_data: Some(request_data),
            action_type,
        }
    }

    async fn get_delete_sql(
        gs: &GlobalState,
        request_data: &RequestData,
        _session_data: &UserSessionData,
    ) -> Result<(String, Vec<String>), NoValueFoundError> {
        let app: &RikdataApplication = gs
            .get_app_for_base_path(&request_data.base_path)
            .map_err(|_err| NoValueFoundError::new("Invalid base path"))?;

        let application_code = app
            .code
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("Invalid application_code"))?
            .as_str();

        //check if menu_path is available
        let menu = global_cache::get_menu(
            &request_data.base_path,
            application_code,
            &request_data.entity_path,
            gs.sqlite_pools.get("local"),
        )
        .await;

        match menu {
            Ok(menu) => {
                if let Some(_fields) = &menu.fields {
                    Ok(DbDelete::get_default_sql(request_data))
                } else {
                    Ok(DbDelete::get_default_sql(request_data))
                }
            }
            Err(_) => Ok(DbDelete::get_default_sql(request_data)),
        }
    }

    async fn get_update_sql(
        gs: &GlobalState,
        request_data: &RequestData,
        session_data: &UserSessionData,
    ) -> Result<(String, Vec<String>), NoValueFoundError> {
        let app: &RikdataApplication = gs
            .get_app_for_base_path(&request_data.base_path)
            .map_err(|_err| NoValueFoundError::new("Invalid base path"))?;

        let application_code = app
            .code
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("Invalid application_code"))?
            .as_str();

        //check if menu_path is available
        let menu = global_cache::get_menu(
            &request_data.base_path,
            application_code,
            &request_data.entity_path,
            gs.sqlite_pools.get("local"),
        )
        .await;

        match menu {
            Ok(menu) => {
                if let Some(fields) = &menu.fields {
                    if !fields.is_empty() {
                        update::get_sql_with_menu(request_data, fields, session_data)
                    } else {
                        update::get_default_sql(request_data, session_data)
                    }
                } else {
                    update::get_default_sql(request_data, session_data)
                }
            }
            Err(_) => update::get_default_sql(request_data, session_data),
        }
    }

    async fn get_insert_sql(
        gs: &GlobalState,
        request_data: &RequestData,
        json_body: &BodyData,
        session_data: &UserSessionData,
    ) -> Result<(String, Vec<String>), NoValueFoundError> {
        let app: &RikdataApplication = gs
            .get_app_for_base_path(&request_data.base_path)
            .map_err(|_err| NoValueFoundError::new("Invalid base path"))?;

        let application_code = app
            .code
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("Invalid application_code"))?
            .as_str();

        //check if menu_path is available
        let menu = global_cache::get_menu(
            &request_data.base_path,
            application_code,
            &request_data.entity_path,
            gs.sqlite_pools.get("local"),
        )
        .await;

        match menu {
            Ok(menu) => {
                if let Some(fields) = &menu.fields {
                    if !fields.is_empty() {
                        let data = insert::get_sql_with_menu(
                            request_data,
                            json_body,
                            fields,
                            session_data,
                        )?;
                        Ok(data)
                    } else {
                        insert::get_default_sql(request_data, json_body, session_data)
                    }
                } else {
                    insert::get_default_sql(request_data, json_body, session_data)
                }
            }
            Err(_) => insert::get_default_sql(request_data, json_body, session_data),
        }
    }
}
