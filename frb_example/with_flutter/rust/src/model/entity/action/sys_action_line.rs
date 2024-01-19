use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{model::db_conn_map::DbConnMapping, select::Select};
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct SysActionLine {
    pub sys_action_line_id: i32,
    pub src_entity_name: Option<String>,
    pub src_entity_id: Option<String>,
    pub sys_action_id: i32,
    pub sequence: i32,
    pub src_table_name: Option<String>,
    pub src_table_id: Option<String>,
    pub dst_table_name: Option<String>,
    pub dst_table_id: Option<String>,
    pub line_action_code: Option<String>,
    pub sys_action_line_id_pass: Option<i32>,
    pub sys_action_line_id_fail: Option<i32>,
    pub sys_action_line_id_onhold: Option<i32>,
    pub activity_path: Option<String>,
    pub description: Option<String>,
    pub is_go_routine: Option<i32>,
    pub created_by: String,
    pub creation_date: NaiveDateTime,
    pub last_updated_by: String,
    pub last_update_date: NaiveDateTime,
}

impl SysActionLine {
    pub async fn find_all(
        conn_mapping: &DbConnMapping,
    ) -> Result<Vec<SysActionLine>, NoValueFoundError> {
        let sql = "SELECT * from sys_action_line";
        let rows = Select::new(conn_mapping).select(sql, &vec![]).await?;
        let sys_action_lines: Result<Vec<SysActionLine>, _> =
            rows.into_iter().map(Self::get_entity).collect();
        sys_action_lines
    }

    pub async fn find_by_id(
        conn_mapping: &DbConnMapping,
        id: &str,
    ) -> Result<SysActionLine, NoValueFoundError> {
        let sql = format!(
            "SELECT * from sys_action_line where sys_action_line_id ='{}' ",
            id
        );
        let data = Select::new(conn_mapping)
            .select(sql.as_str(), &vec![])
            .await?;
        let row = data
            .first()
            .ok_or_else(|| NoValueFoundError::new("Unable to fetch first row"))?
            .to_owned();
        let entity: Result<SysActionLine, NoValueFoundError> = Self::get_entity(row);
        entity
    }

    pub async fn find_by_sql(
        conn_mapping: &DbConnMapping,
        sql: &str,
        params: &Vec<String>,
    ) -> Result<Vec<SysActionLine>, NoValueFoundError> {
        let rows = Select::new(conn_mapping).select(sql, params).await?;
        let sys_action_lines: Result<Vec<SysActionLine>, _> =
            rows.into_iter().map(Self::get_entity).collect();
        sys_action_lines
    }

    fn get_entity(
        row: linked_hash_map::LinkedHashMap<String, String>,
    ) -> Result<SysActionLine, NoValueFoundError> {
        let sys_action_line_id: i32 = row
            .get("sysActionLineId")
            .ok_or_else(|| NoValueFoundError::new("Missing sysActionLineId"))?
            .parse()
            .map_err(|_| NoValueFoundError::new("Invalid sysActionLineId"))?;
        let src_entity_name: Option<String> = row.get("srcEntityName").map(|v| v.to_string());
        let src_entity_id: Option<String> = row.get("srcEntityId").map(|v| v.to_string());
        let sys_action_id: i32 = row
            .get("sysActionId")
            .ok_or_else(|| NoValueFoundError::new("Missing sysActionId"))?
            .parse()
            .map_err(|_| NoValueFoundError::new("Invalid sysActionId"))?;
        let sequence: i32 = row
            .get("sequence")
            .ok_or_else(|| NoValueFoundError::new("Missing sequence"))?
            .parse()
            .map_err(|_| NoValueFoundError::new("Invalid sequence"))?;
        let src_table_name: Option<String> = row.get("srcTableName").map(|v| v.to_string());
        let src_table_id: Option<String> = row.get("srcTableId").map(|v| v.to_string());
        let dst_table_name: Option<String> = row.get("dstTableName").map(|v| v.to_string());
        let dst_table_id: Option<String> = row.get("dstTableId").map(|v| v.to_string());
        let line_action_code: Option<String> = row.get("lineActionCode").map(|v| v.to_string());
        let sys_id_1: Option<&String> = row.get("sysActionLineIdPass");
        let sys_action_line_id_pass = match sys_id_1 {
            Some(val) => {
                let parsed_val = val.parse();
                match parsed_val {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
            None => None,
        };
        let sys_id_1: Option<&String> = row.get("sysActionLineIdFail");
        let sys_action_line_id_fail = match sys_id_1 {
            Some(val) => {
                let parsed_val = val.parse();
                match parsed_val {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
            None => None,
        };
        let sys_id_1: Option<&String> = row.get("sysActionLineIdOnhold");
        let sys_action_line_id_onhold = match sys_id_1 {
            Some(val) => {
                let parsed_val = val.parse();
                match parsed_val {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
            None => None,
        };
        let activity_path: Option<String> = row.get("activityPath").map(|v| v.to_string());
        let description: Option<String> = row.get("description").map(|v| v.to_string());
        let sys_id_1: Option<&String> = row.get("isGoRoutine");
        let is_go_routine = match sys_id_1 {
            Some(val) => {
                let parsed_val = val.parse();
                match parsed_val {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
            None => None,
        };
        let created_by: String = row
            .get("createdBy")
            .ok_or_else(|| NoValueFoundError::new("Missing createdBy"))?
            .to_string();
        let str1 = row
            .get("creationDate")
            .ok_or_else(|| NoValueFoundError::new("Missing creationDate"))?;
        let creation_date: NaiveDateTime =
            NaiveDateTime::parse_from_str(str1.as_str(), "%Y-%m-%d %H:%M:%S")
                .map_err(|_| NoValueFoundError::new("Invalid creationDate"))?;
        let last_updated_by: String = row
            .get("lastUpdatedBy")
            .ok_or_else(|| NoValueFoundError::new("Missing lastUpdatedBy"))?
            .to_string();
        let str1 = row
            .get("lastUpdateDate")
            .ok_or_else(|| NoValueFoundError::new("Missing lastUpdateDate"))?;
        let last_update_date: NaiveDateTime =
            NaiveDateTime::parse_from_str(str1.as_str(), "%Y-%m-%d %H:%M:%S")
                .map_err(|_| NoValueFoundError::new("Invalid lastUpdateDate"))?;
        Ok(SysActionLine {
            sys_action_line_id,
            src_entity_name,
            src_entity_id,
            sys_action_id,
            sequence,
            src_table_name,
            src_table_id,
            dst_table_name,
            dst_table_id,
            line_action_code,
            sys_action_line_id_pass,
            sys_action_line_id_fail,
            sys_action_line_id_onhold,
            activity_path,
            description,
            is_go_routine,
            created_by,
            creation_date,
            last_updated_by,
            last_update_date,
        })
    }
}

