use crate::db::model::db_type::DbType;
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsRustInput {
    pub data: serde_json::Value,
    pub request: serde_json::Value,
    #[serde(rename = "pathParamValues", default)]
    path_param_values: String,
    params: String,
}

impl JsRustInput {
    pub fn new(data: serde_json::Value, request: serde_json::Value, path_param_values: String, params: String) -> Self {
        Self {
            data,
            request,
            path_param_values,
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsDbInput {
    pub sql: String,
    pub db_type: DbType,
    pub conn_name: String,
}

impl JsDbInput {
    pub fn new(sql: String, db_type: DbType, conn_name: String) -> Self {
        Self {
            sql,
            db_type,
            conn_name,
        }
    }
}
