use chrono::{ NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{model::db_conn_map::DbConnMapping, select::Select};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
        pub struct SysAction {pub sys_action_id : i32,
pub src_entity_name : Option<String>,
pub src_entity_id : Option<String>,
pub action_code : String,
pub sequence : i32,
pub next_action_code : Option<String>,
pub action_name : String,
pub action_type : String,
pub description : Option<String>,
pub class_name : Option<String>,
pub method_name : Option<String>,
pub next_line_seq_pass : Option<i32>,
pub next_line_seq_fail : Option<i32>,
pub next_line_seq_onhold : Option<i32>,
pub activity_path : Option<String>,
pub created_by : String,
pub creation_date : NaiveDateTime,
pub last_updated_by : String,
pub last_update_date : NaiveDateTime,

}


impl SysAction {
    pub async fn find_all(
        conn_mapping: &DbConnMapping,
    ) -> Result<Vec<SysAction>, NoValueFoundError> {
        let sql = "SELECT * from sys_action";
        let rows = Select::new(conn_mapping).select(sql, &vec![]).await?;
        let sys_actions: Result<Vec<SysAction>, _> =
            rows.into_iter().map(Self::get_entity).collect();
        sys_actions
    }

    pub async fn find_by_id(
        conn_mapping: &DbConnMapping,
        id: &str,
    ) -> Result<SysAction, NoValueFoundError> {
        let sql = format!(
            "SELECT * from sys_action where sys_action_id ='{}' ",
            id
        );
        let data = Select::new(conn_mapping)
            .select(sql.as_str(), &vec![])
            .await?;
        let row = data
        .first()
        .ok_or_else(|| NoValueFoundError::new("Unable to fetch first row"))?
        .to_owned();
        let entity: Result<SysAction, NoValueFoundError> = Self::get_entity(row);
        entity
    }

    pub async fn find_by_sql(
        conn_mapping: &DbConnMapping,
        sql: &str,
        params: &Vec<String>,
    ) -> Result<Vec<SysAction>, NoValueFoundError> {
        let rows = Select::new(conn_mapping).select(sql, params).await?;
        let sys_actions: Result<Vec<SysAction>, _> =
            rows.into_iter().map(Self::get_entity).collect();
        sys_actions
    }

    
fn get_entity(
            row: linked_hash_map::LinkedHashMap<String, String>,
        ) -> Result<SysAction , NoValueFoundError> { let sys_action_id: i32 = row
                    .get("sysActionId")
                    .ok_or_else(|| NoValueFoundError::new("Missing sysActionId"))?
                    .parse()
                    .map_err(|_| NoValueFoundError::new("Invalid sysActionId"))?; let src_entity_name: Option<String> = row.get("srcEntityName").map(|v| v.to_string());let src_entity_id: Option<String> = row.get("srcEntityId").map(|v| v.to_string());let action_code: String = row.get("actionCode").ok_or_else(|| NoValueFoundError::new("Missing actionCode"))?
                    .to_string();let sequence: i32 = row
                    .get("sequence")
                    .ok_or_else(|| NoValueFoundError::new("Missing sequence"))?
                    .parse()
                    .map_err(|_| NoValueFoundError::new("Invalid sequence"))?; let next_action_code: Option<String> = row.get("nextActionCode").map(|v| v.to_string());let action_name: String = row.get("actionName").ok_or_else(|| NoValueFoundError::new("Missing actionName"))?
                    .to_string();let action_type: String = row.get("actionType").ok_or_else(|| NoValueFoundError::new("Missing actionType"))?
                    .to_string();let description: Option<String> = row.get("description").map(|v| v.to_string());let class_name: Option<String> = row.get("className").map(|v| v.to_string());let method_name: Option<String> = row.get("methodName").map(|v| v.to_string());
let sys_id_1 : Option<&String> = row.get("nextLineSeqPass");
let next_line_seq_pass= match sys_id_1 {
                        Some(val) => {
                            let parsed_val = val.parse();
                            match parsed_val {
                                Ok(val) => Some(val),
                                Err(_) => None,
                            }
                        }
                        None => None,
                    }; 
let sys_id_1 : Option<&String> = row.get("nextLineSeqFail");
let next_line_seq_fail= match sys_id_1 {
                        Some(val) => {
                            let parsed_val = val.parse();
                            match parsed_val {
                                Ok(val) => Some(val),
                                Err(_) => None,
                            }
                        }
                        None => None,
                    }; 
let sys_id_1 : Option<&String> = row.get("nextLineSeqOnhold");
let next_line_seq_onhold= match sys_id_1 {
                        Some(val) => {
                            let parsed_val = val.parse();
                            match parsed_val {
                                Ok(val) => Some(val),
                                Err(_) => None,
                            }
                        }
                        None => None,
                    }; let activity_path: Option<String> = row.get("activityPath").map(|v| v.to_string());let created_by: String = row.get("createdBy").ok_or_else(|| NoValueFoundError::new("Missing createdBy"))?
                    .to_string(); let str1 = row
                    .get("creationDate")
                    .ok_or_else(|| NoValueFoundError::new("Missing creationDate"))?; 
let creation_date: NaiveDateTime =
                  NaiveDateTime::parse_from_str(str1.as_str(), "%Y-%m-%d %H:%M:%S")
                      .map_err(|_| NoValueFoundError::new("Invalid creationDate"))?;let last_updated_by: String = row.get("lastUpdatedBy").ok_or_else(|| NoValueFoundError::new("Missing lastUpdatedBy"))?
                    .to_string(); let str1 = row
                    .get("lastUpdateDate")
                    .ok_or_else(|| NoValueFoundError::new("Missing lastUpdateDate"))?; 
let last_update_date: NaiveDateTime =
                  NaiveDateTime::parse_from_str(str1.as_str(), "%Y-%m-%d %H:%M:%S")
                      .map_err(|_| NoValueFoundError::new("Invalid lastUpdateDate"))?;
 Ok(SysAction { sys_action_id,src_entity_name,src_entity_id,action_code,sequence,next_action_code,action_name,action_type,description,class_name,method_name,next_line_seq_pass,next_line_seq_fail,next_line_seq_onhold,activity_path,created_by,creation_date,last_updated_by,last_update_date, }) 
}

}


