use dashmap::DashMap;
use lazy_static::lazy_static; // 1.4.0
use sqlx::{self, Pool};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::app::js::entity::js_object::JsPrimaryObject;
use crate::app::js::entity::js_trigger_point::JsTriggerPoint;
use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::configuration::Settings;
use crate::db::model::db_conn_map::DbConnMapping;
use crate::db::model::db_type::DbType;
use crate::model::common::adv::adv_menu_path::AdvMenuPath;
use crate::model::common::local::rikdata_application::RikdataApplication;
use crate::model::entity::action::sys_action::SysAction;
use crate::model::entity::adv::adv_sys_action::AdvSysAction;
use crate::model::state::global_state::GlobalState;

//Used only for testing use global state for all production usage
lazy_static! {
    static ref GC: Arc<Mutex<GlobalCache>> = Arc::new(Mutex::new(GlobalCache::new()));
    static ref GC_MENU_MAP : DashMap<String, AdvMenuPath> = DashMap::with_capacity(1000); //key is the base_path__menu_path_code (ex: ierp__PoHeaderEv)
    //Patch action map contains information about action lines against a specific action
    //if the action is line_create_inv_transaction_using_sd_delivery_line_for_invt_v, then lines system must process
    #[derive(Debug)]
    static ref GC_PATCH_ACTION_MAP : DashMap<String, AdvSysAction> = DashMap::with_capacity(200);//key is the base_path__action_code (ex: ierp__convert_to_sd_so_header)
    static ref GC_JS_OBJECTS : DashMap<String, JsPrimaryObject> = DashMap::with_capacity(500); //key is the basePath__entityPath__triggerPoint (ex: ierp__PoHeaderEv__beforeGet)
    static ref GC_USER_DATA : DashMap<String, UserSessionData> = DashMap::with_capacity(100);//key is the user_id
    //Caches all the action available for an entity
    #[derive(Debug)]
    static ref GC_ENTITY_ACTION_MAP : DashMap<String, Vec<SysAction>> = DashMap::with_capacity(1000); //key is the base_path__entity_path (ex:ierp__SdSoHeaderEv)

}

#[derive(Debug)]
pub struct GlobalCache {
    settings: Option<Settings>, //settings from config.yml
    applications: Option<Vec<RikdataApplication>>,
    base_path_db_type_map: HashMap<String, DbType>, //key is the base_path
    base_path_db_conn_name_map: HashMap<String, String>, //key is the base_path
    gs: Arc<Mutex<Option<GlobalState>>>,
}

impl Default for GlobalCache {
    fn default() -> Self {
        GlobalCache::new()
    }
}

impl GlobalCache {
    pub fn new() -> Self {
        Self {
            settings: None,
            applications: None,
            base_path_db_type_map: HashMap::new(),
            base_path_db_conn_name_map: HashMap::new(),
            gs: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_applications(apps: Vec<RikdataApplication>) -> Result<bool, NoValueFoundError> {
        let mut lock = GC
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to set applications"))?;
        apps.iter().for_each(|f| {
            if let Some(base_path) = &f.base_path {
                if let Some(db_type) = &f.db_type {
                    lock.base_path_db_type_map
                        .insert(base_path.to_string(), DbType::from_string(db_type));
                }
                if let Some(db_conn_name) = &f.db_conn_name {
                    lock.base_path_db_conn_name_map
                        .insert(base_path.to_string(), db_conn_name.to_string());
                }
            }
        });
        lock.applications = Some(apps);
        Ok(true)
    }

    pub fn get_base_path(db_conn_name: &str) -> Result<String, NoValueFoundError> {
        let lock = GC
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to get base_path_db_conn_name_map"))?;

        let base_path = lock
            .base_path_db_conn_name_map
            .iter()
            .find(|(_, conn_name)| conn_name.eq_ignore_ascii_case(db_conn_name))
            .map(|(base_path, _)| base_path.to_string())
            .ok_or_else(|| NoValueFoundError::new(
                format!("No base path found for the db_conn_name: {}", db_conn_name).as_str(),
            ))?;
        Ok(base_path)
        // match base_path {
        //     Some(path) => Ok(path),
        //     None => Err(NoValueFoundError::new(
        //         format!("No base path found for the db_conn_name: {}", db_conn_name).as_str(),
        //     )),
        // }
    }

    //first set caches dependent on global states
    pub async fn set_global_state(state: GlobalState) -> Result<bool, NoValueFoundError> {
        Self::set_patch_actions_map(&state).await?;
        let mut lock = GC
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to set GlobalState"))?;
        lock.gs = Arc::new(Mutex::new(Some(state)));
        Ok(true)
    }

    pub fn get_global_state() -> Result<GlobalState, NoValueFoundError> {
        let lock = GC
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to lock for global state 1"))?;
        let gs = lock
            .gs
            .as_ref()
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to lock for global state 22"));
        let gs_val =
            gs.map_err(|_err| NoValueFoundError::new("Unable to lock for global state 21"))?;
        let gs_val2 = gs_val
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("No global state value found"))?;
        Ok(gs_val2.to_owned())
    }

    pub fn set_settings(settings: Settings) -> Result<bool, NoValueFoundError> {
        let mut lock = GC
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to set settings"))?;
        lock.settings = Some(settings);
        Ok(true)
    }

    pub fn get_settings() -> Result<Settings, NoValueFoundError> {
        let lock = GC
            .lock()
            .map_err(|_err| NoValueFoundError::new("Unable to get settings"))?;
        let settings = lock
            .settings
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("get_settings in gc"))?;
        Ok(settings.to_owned())
    }

    pub fn get_js_objects(
        base_path: &str,
        entity_path: &str,
        trigger_point: &JsTriggerPoint,
    ) -> Option<JsPrimaryObject> {
        let key =
            base_path.to_string() + "__" + entity_path + "__" + trigger_point.to_string().as_str();
        GC_JS_OBJECTS.get(&key).map(|data| data.to_owned())
    }

    pub fn set_js_objects() -> Result<bool, NoValueFoundError> {
        let js_objects = JsPrimaryObject::init_from_asset()?;
        for (k, v) in js_objects {
            GC_JS_OBJECTS.insert(k, v);
        }
        //GC_JS_OBJECTS.extend( js_objects.clone().iter());
        Ok(true)
    }

    pub fn set_user_session_data(data: UserSessionData) -> Result<bool, NoValueFoundError> {
        GC_USER_DATA.insert(data.id.to_owned(), data);
        Ok(true)
    }

    pub fn get_user_session_data(user_id: &str) -> Option<UserSessionData> {
        GC_USER_DATA.get(user_id).map(|val| val.to_owned())
    }

    pub async fn set_patch_actions_map(state: &GlobalState) -> Result<bool, NoValueFoundError> {
        let pools = state.conn_pools.lock().await;
        for base_path in pools.keys() {
            if base_path.eq_ignore_ascii_case("local") {
                continue;
            }
            let pool = pools
                .get(base_path)
                .ok_or_else(|| NoValueFoundError::new("Unable to find pool for base_path"))?;
            let adv_sys_actions = AdvSysAction::find_all(base_path, pool).await;
            match adv_sys_actions {
                Ok(adv_sys_actions) => {
                    for (key, val) in adv_sys_actions {
                        GC_PATCH_ACTION_MAP.insert(key, val);
                    }
                }
                Err(_err) => {
                    log::error!("No actions found for path {}", base_path);
                }
            }
        }
        Ok(true)
    }

    pub async fn get_patch_action(base_path: &str, action_code: &str) -> Option<AdvSysAction> {
        let key = base_path.to_string() + "__" + action_code;
        GC_PATCH_ACTION_MAP.get(&key).map(|data| data.to_owned())
    }
    pub async fn get_entity_actions(
        base_path: &str,
        entity_path: &str,
        sql: &str,
        conn_mapping: &DbConnMapping,
    ) -> Result<Vec<SysAction>, NoValueFoundError> {
        let key = base_path.to_string() + "__" + entity_path;
        match GC_ENTITY_ACTION_MAP.get(&key) {
            Some(sys_actions) => Ok(sys_actions.to_owned()),
            None => {
                let sys_action: Vec<SysAction> =
                    SysAction::find_by_sql(conn_mapping, sql, &vec![]).await?;
                GC_ENTITY_ACTION_MAP.insert(key, sys_action.clone());
                Ok(sys_action)
            }
        }
    }
}
pub async fn get_menu(
    base_path: &str,
    application_code: &str,
    menu_path_code: &str,
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
) -> Result<AdvMenuPath, NoValueFoundError> {
    let key = base_path.to_string() + "__" + menu_path_code;
    match GC_MENU_MAP.get(&key) {
        Some(adv_menu) => Ok(adv_menu.to_owned()),
        None => {
            let adv_menu = AdvMenuPath::new(
                application_code.to_string(),
                menu_path_code.to_string(),
                sqlite_pool,
            )
            .await?;
            GC_MENU_MAP.insert(key, adv_menu.clone());
            Ok(adv_menu)
        }
    }
}

mod tests {}
