use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use crate::{
    app::{system::error::no_value::NoValueFoundError, cache::global_cache::GlobalCache},
    db::model::db_conn_map::DbConnMapping,
    model::entity::action::{sys_action::SysAction, sys_action_line::SysActionLine},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AdvSysAction {
    pub sys_action: SysAction,
    pub sys_action_lines: Vec<SysActionLine>,
}

impl AdvSysAction {
    pub async fn find_all(
        base_path: &str,
        conn_mapping: &DbConnMapping,
    ) -> Result<HashMap<String, AdvSysAction>, NoValueFoundError> {
        let mut ret_map: HashMap<String, AdvSysAction> = HashMap::new();
        let sys_actions = SysAction::find_all(conn_mapping).await?;
        let sys_action_lines: Vec<SysActionLine> = SysActionLine::find_all(conn_mapping).await?;
        let all_ids_with_line: HashSet<i32> =
            sys_action_lines.iter().map(|f| f.sys_action_id).collect();

        for action in sys_actions {
            if all_ids_with_line.contains(&action.sys_action_id) {
                let sys_action_lines1: Vec<SysActionLine> = sys_action_lines
                    .iter()
                    .filter(|l| l.sys_action_id.eq(&action.sys_action_id)).cloned()
                    .collect();
                let key = base_path.to_string() + "__" + action.action_code.clone().as_str()  ;
                let sdv_sys_action: AdvSysAction = AdvSysAction {
                    sys_action: action,
                    sys_action_lines: sys_action_lines1,
                };
                
                ret_map.insert(key, sdv_sys_action);
            }
        }

        Ok(ret_map)
    }

    pub async fn get_entity_actions(
        base_path: &str,
        entity_path: &str,
        conn_mapping: &DbConnMapping,
    ) -> Result<Vec<SysAction>, NoValueFoundError> {
        let sql = format!(
            "SELECT * FROM sys_action_ev 
       where 1 = 1 
       AND  vv_path_url ='{entity_path}' ORDER BY sequence ASC "
        );
        GlobalCache::get_entity_actions(base_path, entity_path, &sql, conn_mapping).await
    }
}


