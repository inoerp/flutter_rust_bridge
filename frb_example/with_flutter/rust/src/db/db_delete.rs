use super::{
    super::app::utils::istr as istr_utils,
};
use crate::{
    app::api::request::{model::request_data::RequestData},
};


pub struct DbDelete;

impl DbDelete {
    
        pub fn get_default_sql(request_data: &RequestData) -> (String, Vec<String>) {
        //use menu fields
        let mut params: Vec<String> = Vec::new();
        let mut sql = "DELETE FROM ".to_string() + request_data.entity_base_table.as_str();
    
        //add where clauses
        if request_data.url_data.is_empty() {
            sql += " WHERE 1 = 2 ";
            return (sql, params);
        }
    
        sql +=  " WHERE 1 = 1 ";
    
        for data in &request_data.url_data {
            if data.value.as_str() != "" && data.value.as_str().to_lowercase() != "null" {
                sql = sql
                    + " AND  "
                    + istr_utils::pascal_to_snake(data.key.as_str()).as_str()
                    + " "
                    + data.condition.to_db_string()
                    + " ? ";
                params.push(data.value.clone());
            }
        }
        (sql, params)
    }
    
}

