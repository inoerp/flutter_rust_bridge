use crate::app::api::url::url_data as api_url;
use super::super::app::utils::istr as istr_utils;
use crate::model::state::global_state::GlobalState;
use crate::{app::api::request::model::request_data::RequestData};
use crate::model::data::condition;
use base64::{Engine as _,  engine::{ general_purpose}};

#[derive(Debug, Clone)]
pub struct QueryData {
    pub sql: String,
    pub params: Vec<String>, //sql parameters
    pub request_data: RequestData,
}

//dont delete
pub enum QueryParams<'a> {
    None,
    Integers(Vec<i32>),
    Strings(Vec<&'a str>),
}

impl QueryData {
    pub fn new(request_data: RequestData, sql: String, params: Vec<String>) -> Self {
        QueryData {
            request_data,
            sql,
            params,
        }
    }

    pub fn init_from_request_data(gs: &GlobalState, request_data: RequestData) -> Self {
        let (sql, params) = Self::get_sql_query(gs, &request_data);
        let qd: QueryData = QueryData {
            request_data,
            sql,
            params,
        };
        qd
    }

    fn get_sql_for_query_by_sql( rd: &RequestData) -> (String, Vec<String>) {
       
           let mut sql = "".to_string();       
         for data in &rd.url_data{
            if data.key.eq_ignore_ascii_case("sql"){
                let sql1 = data.value.to_owned();
                let bytes = general_purpose::STANDARD_NO_PAD.decode(sql1).expect("unable to decode");
                sql = String::from_utf8_lossy(&bytes).to_string();
            }
         }

        (sql, vec![])
    }

    //TODO enable bind params
    //Get the field details from menu path and add the params in a enum
    fn get_sql_query(gs: &GlobalState, rd: &RequestData) -> (String, Vec<String>) {
        if rd.entity_table.eq_ignore_ascii_case("query_by_sql") {
            return Self::get_sql_for_query_by_sql(rd);
        }

        let url_data: &Vec<api_url::UrlData> = &rd.url_data;
        let entity: &str = &rd.entity_table;
        let mut limit = gs.settings.app_settings.default_query_limit;
        let mut offset = 0;
        let params: Vec<String> = Vec::new();
        let mut sql = format!("SELECT * FROM {entity}  WHERE 1 = 1 ");
        if url_data.is_empty() {
            return (sql, params);
        }

        for data in url_data {
            match data.key.to_lowercase().as_str() {
                "limit" => {
                    let limit_val = data.value.parse();
                    if let Ok(val) = limit_val {
                        limit = val;
                    }
                }
                "offset" => {
                    let offset_val = data.value.parse();
                    if let Ok(val) = offset_val {
                        offset = val;
                    }
                }
                _ => {
                    if data.value.as_str() != "" && data.value.as_str().to_lowercase() != "null" {
                        match data.condition {
                            condition::Condition::StartsWith => {
                                sql = sql
                                    + " AND  "
                                    + istr_utils::pascal_to_camel(data.key.as_str()).as_str()
                                    + " "
                                    + data.condition.to_db_string()
                                    + "'"
                                    + data.value.as_str()
                                    + "%'";
                            }
                            condition::Condition::EndsWith => {
                                sql = sql
                                    + " AND  "
                                    + istr_utils::pascal_to_camel(data.key.as_str()).as_str()
                                    + " "
                                    + data.condition.to_db_string()
                                    + "'%"
                                    + data.value.as_str()
                                    + "'";
                            }
                            condition::Condition::Like
                            | condition::Condition::NotLike
                            | condition::Condition::Contains
                            | condition::Condition::DoesNotContains => {
                                sql = sql
                                    + " AND  "
                                    + istr_utils::pascal_to_camel(data.key.as_str()).as_str()
                                    + " "
                                    + data.condition.to_db_string()
                                    + "'%"
                                    + data.value.as_str()
                                    + "%'";
                            }
                            _ => {
                                sql = sql
                                    + " AND  "
                                    + istr_utils::pascal_to_camel(data.key.as_str()).as_str()
                                    + " "
                                    + data.condition.to_db_string()
                                    + "'"
                                    + data.value.as_str()
                                    + "'";
                            }
                        }

                        //params.push(data.value);
                    }
                }
            }
        }
        sql =
            sql + " limit " + limit.to_string().as_str() + " offset " + offset.to_string().as_str();
        (sql, params)
    }

    //dont delete
    pub fn get_sql_query_with_params(
        url_data: Vec<api_url::UrlData>,
        entity: &str,
    ) -> (String, Vec<String>) {
        let mut limit = 10;
        let mut offset = 0;
        let mut params: Vec<String> = Vec::new();
        let mut sql = format!("SELECT * FROM {entity}  WHERE 1 = 1 ");
        if url_data.is_empty() {
            return (sql, params);
        }

        for data in url_data {
            match data.key.to_lowercase().as_str() {
                "limit" => {
                    let limit_val = data.value.parse();
                    if let Ok(val) = limit_val {
                        limit = val;
                    }
                }
                "offset" => {
                    let offset_val = data.value.parse();
                    if let Ok(val) = offset_val {
                        offset = val;
                    }
                }
                _ => {
                    if data.value.as_str() != "" && data.value.as_str().to_lowercase() != "null" {
                        sql = sql
                            + " AND  "
                            + istr_utils::pascal_to_camel(data.key.as_str()).as_str()
                            + " "
                            + data.condition.to_db_string()
                            + " ? ";
                        params.push(data.value);
                    }
                }
            }
        }
        sql =
            sql + "limit " + limit.to_string().as_str() + " offset " + offset.to_string().as_str();
        (sql, params)
    }
}
