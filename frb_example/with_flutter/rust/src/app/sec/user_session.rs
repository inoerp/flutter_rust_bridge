use crate::app::cache::global_cache::GlobalCache;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::model::common::adv::adv_sec_user;
use crate::model::common::local::rd_sec_user::RdSecUser;
use chrono::Utc;
use futures::future;
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionData {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub token: String,
    pub id: String,
    #[serde(rename = "userDetails")]
    pub user_details: RdSecUser,
    #[serde(rename = "roleAccess")]
    pub role_access: HashMap<String, String>,
    #[serde(rename = "buOrgs")]
    pub bu_orgs: HashMap<String, String>,
    #[serde(rename = "invOrgs")]
    pub inv_orgs: HashMap<String, String>,
    #[serde(rename = "orgCodes")]
    pub org_codes: HashMap<String, String>,
    #[serde(rename = "loggedInAt")]
    pub logged_in_at: String, //TODO use DateTime
    pub status: UserSessionStatus,
}

impl UserSessionData {
    pub async fn init_for_user(
        pool: Option<&Pool<sqlx::Sqlite>>,
        token: &str,
        user: &RdSecUser,
    ) -> Result<Self, NoValueFoundError> {
        let mut filter_user = user.clone();
        filter_user.password = None;
        let user_id = match &user.id {
            Some(name) => name.to_string(),
            None => String::new(),
        };
        let user_roles = adv_sec_user::get_user_role(pool, &user_id);
        let bu_lovs = adv_sec_user::get_user_org_data(pool, "bu_access_group_lov", &user_id);
        let inv_lovs = adv_sec_user::get_user_org_data(pool, "inv_access_group_lov", &user_id);
        let org_code_lovs = adv_sec_user::get_user_org_data(pool, "org_code_lov", &user_id);

        let futures = future::join4(user_roles, bu_lovs, inv_lovs, org_code_lovs);
        let (user_roles1, bu_lovs1, inv_lovs1, org_code_lovs1) = futures.await;

        let user_name = match &user.username {
            Some(name) => name.to_string(),
            None => String::new(),
        };

        let role_access: HashMap<String, String> = match &user_roles1 {
            Ok(val) => val.to_owned(),
            Err(_err) => HashMap::<String, String>::new(),
        };
        let bu_orgs: HashMap<String, String> = match &bu_lovs1 {
            Ok(val) => val.to_owned(),
            Err(_err) => HashMap::<String, String>::new(),
        };
        let inv_orgs: HashMap<String, String> = match &inv_lovs1 {
            Ok(val) => val.to_owned(),
            Err(_err) => HashMap::<String, String>::new(),
        };
        let org_codes: HashMap<String, String> = match &org_code_lovs1 {
            Ok(val) => val.to_owned(),
            Err(_err) => HashMap::<String, String>::new(),
        };
        let session_data = Self {
            user_name,
            token: token.to_string(),
            id: user_id.clone(),
            user_details: filter_user,
            role_access,
            bu_orgs,
            inv_orgs,
            org_codes,
            logged_in_at: Utc::now().to_string(),
            status: UserSessionStatus::Active,
        };
        GlobalCache::set_user_session_data(session_data)?;
        let data = GlobalCache::get_user_session_data(&user_id)
            .ok_or_else(|| NoValueFoundError::new("Unable to create user session"))?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserSessionStatus {
    Active,
    Expired,
    LoggedOut,
}
