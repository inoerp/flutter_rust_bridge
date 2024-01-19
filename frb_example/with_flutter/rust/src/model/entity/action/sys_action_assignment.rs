use chrono::{  NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{model::db_conn_map::DbConnMapping, select::Select};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
        pub struct SysActionAssignment {pub sys_action_assignment_id : i32,
pub src_entity_name : Option<String>,
pub src_entity_id : Option<String>,
pub sys_action_code : String,
pub path_url : String,
pub created_by : String,
pub creation_date : NaiveDateTime,
pub last_updated_by : String,
pub last_update_date : NaiveDateTime,

}


impl SysActionAssignment {
    pub async fn find_all(
        conn_mapping: &DbConnMapping,
    ) -> Result<Vec<SysActionAssignment>, NoValueFoundError> {
        let sql = "SELECT * from sys_action_assignment";
        let rows = Select::new(conn_mapping).select(sql, &vec![]).await?;
        let sys_action_assignments: Result<Vec<SysActionAssignment>, _> =
            rows.into_iter().map(Self::get_entity).collect();
        sys_action_assignments
    }

    pub async fn find_by_id(
        conn_mapping: &DbConnMapping,
        id: &str,
    ) -> Result<SysActionAssignment, NoValueFoundError> {
        let sql = format!(
            "SELECT * from sys_action_assignment where sys_action_assignment_id ='{}' ",
            id
        );
        let data = Select::new(conn_mapping)
            .select(sql.as_str(), &vec![])
            .await?;
        let row = data
        .first()
        .ok_or_else(|| NoValueFoundError::new("Unable to fetch first row"))?
        .to_owned();
        let entity: Result<SysActionAssignment, NoValueFoundError> = Self::get_entity(row);
        entity
    }

    pub async fn find_by_sql(
        conn_mapping: &DbConnMapping,
        sql: &str,
        params: &Vec<String>,
    ) -> Result<Vec<SysActionAssignment>, NoValueFoundError> {
        let rows = Select::new(conn_mapping).select(sql, params).await?;
        let sys_action_assignments: Result<Vec<SysActionAssignment>, _> =
            rows.into_iter().map(Self::get_entity).collect();
        sys_action_assignments
    }

    
fn get_entity(
            row: linked_hash_map::LinkedHashMap<String, String>,
        ) -> Result<SysActionAssignment , NoValueFoundError> { let sys_action_assignment_id: i32 = row
                    .get("sysActionAssignmentId")
                    .ok_or_else(|| NoValueFoundError::new("Missing sysActionAssignmentId"))?
                    .parse()
                    .map_err(|_| NoValueFoundError::new("Invalid sysActionAssignmentId"))?; let src_entity_name: Option<String> = row.get("srcEntityName").map(|v| v.to_string());let src_entity_id: Option<String> = row.get("srcEntityId").map(|v| v.to_string());let sys_action_code: String = row.get("sysActionCode").ok_or_else(|| NoValueFoundError::new("Missing sysActionCode"))?
                    .to_string();let path_url: String = row.get("pathUrl").ok_or_else(|| NoValueFoundError::new("Missing pathUrl"))?
                    .to_string();let created_by: String = row.get("createdBy").ok_or_else(|| NoValueFoundError::new("Missing createdBy"))?
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
 Ok(SysActionAssignment { sys_action_assignment_id,src_entity_name,src_entity_id,sys_action_code,path_url,created_by,creation_date,last_updated_by,last_update_date, }) 
}

}


