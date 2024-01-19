
use crate::app::api::request::model::request_type::RequestType;
use crate::app::api::url::url_data::{self, UrlData};
use crate::app::sec::auth;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::app::utils::istr as istr_utils;
use crate::model::common::local::rikdata_application::RikdataApplication;
use crate::model::entity::patch_action::PatchActionType;
use crate::model::state::global_state::GlobalState;

use actix_web::HttpRequest;
use actix_web::{http, Error};

use super::body_data::BodyData;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestData {
    pub base_path: String,            //ierp
    pub entity_path: String,          //PoHeaderEv
    pub entity_table: String,         //po_header_ev
    pub entity_base_table: String,    //ex: po_header
    pub action_path: PatchActionType, //copy
    pub url_data: Vec<UrlData>,       //data in the url PoHeaderEv?poHeaderId=65
    pub body_data: BodyData,
    pub request_type: RequestType,
    pub application: RikdataApplication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDataSimple {
    pub base_path: String,         //ierp
    pub entity_path: String,       //PoHeaderEv
    pub entity_table: String,      //po_header_ev
    pub entity_base_table: String, //ex: po_header
    pub action_path: String,       //copy
    pub application_code: String,
}

//dont delete
pub enum QueryParams<'a> {
    None,
    Integers(Vec<i32>),
    Strings(Vec<&'a str>),
}

impl RequestData {
    pub fn new(
        gs: &GlobalState,
        base_path: String,
        entity_path: String,
        url_data: Vec<UrlData>,
        request_type: RequestType,
    ) -> Result<Self, Error> {
        let entity_table = istr_utils::pascal_to_camel(&entity_path);
        let app: &RikdataApplication = gs.get_app_for_base_path(&base_path)?;
        let rd = RequestData {
            base_path,
            entity_base_table: Self::get_base_table(&entity_table),
            entity_table,
            entity_path,
            action_path: PatchActionType::None,
            url_data,
            body_data: BodyData::None,
            request_type,
            application: app.clone(),
        };
        Ok(rd)
    }

    pub fn init_from_http_request_for_config(
        req: &HttpRequest,
        gs: &GlobalState,
        body_data: BodyData,
    ) -> Result<Self, Error> {
        Self::init_from_http_request_with_base_path(req, gs, body_data, "local")
    }

    fn init_from_http_request_with_base_path(
        req: &HttpRequest,
        gs: &GlobalState,
        mut body_data: BodyData,
        base_path: &str,
    ) -> Result<Self, Error> {

        let method: &http::Method = req.method();
        let request_type = RequestType::get_type_from_method(method);

        let possible_entity_path = req
            .match_info()
            .get("entityName")
            .ok_or_else(|| NoValueFoundError::new("Invalid entity name"))?;
        let entity_path: &str;
        let entity_table: String;
        let mut q = req.query_string();
        if possible_entity_path.contains('(') && possible_entity_path.ends_with(')') {
            let splitted_val: Vec<&str> = possible_entity_path.split('(').collect();
            if let Some(val) = splitted_val.first() {
                entity_path = val;
                entity_table = istr_utils::pascal_to_camel(val);
                if let Some(val2) = splitted_val.get(1) {
                    let len = val2.len() - 1;
                    q = &val2[..len];
                }
            } else {
                entity_table = istr_utils::pascal_to_camel(possible_entity_path);
                entity_path = possible_entity_path;
            }
        } else {
            entity_table = istr_utils::pascal_to_camel(possible_entity_path);
            entity_path = possible_entity_path;
        }
        //update body_data
        if base_path.to_lowercase() == "local" && entity_table == "rd_sec_user" {
            body_data = Self::update_body_data(&body_data, &entity_table)?;
        }

        let url_data: Vec<UrlData> = url_data::get_params_from_str(q);
        let app: &RikdataApplication = gs.get_app_for_base_path(base_path)?;
        let qd: RequestData = RequestData {
            entity_base_table: Self::get_base_table(&entity_table),
            entity_table: String::from(&entity_table),
            entity_path: String::from(entity_path),
            base_path: String::from(base_path),
            action_path: PatchActionType::None,
            url_data,
            body_data,
            request_type,
            application: app.clone(),
        };
        Ok(qd)
    }


    pub fn init_from_http_request(
        req: &HttpRequest,
        gs: &GlobalState,
        body_data: BodyData,
    ) -> Result<Self, Error> {
        let base_path = req
            .match_info()
            .get("basePath")
            .ok_or_else(|| NoValueFoundError::new("Invalid base path"))?;
        Self::init_from_http_request_with_base_path(req, gs, body_data, base_path)
    }

    pub fn init_from_http_request_for_action(
        req: &HttpRequest,
        gs: &GlobalState,
        body_data: BodyData,
        action_path: PatchActionType,
    ) -> Result<Self, Error> {
        let mut qd = Self::init_from_http_request(req, gs, body_data)?;
        qd.action_path = action_path;
        Ok(qd)
    }

    fn update_body_data(body_data: &BodyData, entity_table: &str) -> Result<BodyData, Error> {
        if entity_table == "rd_sec_user" {
            if let BodyData::SingleItem(body) = body_data {
                if body.contains_key("password") {
                    let mut body2 = body.clone();
                    let password = body.get("password").ok_or_else(|| {
                        NoValueFoundError::new("Unable to fetch password from body")
                    })?;
                    let hashed_password = auth::get_hashed_password(password);
                    body2.insert("password".to_string(), hashed_password);
                    let body_data = BodyData::SingleItem(body2);
                    return Ok(body_data);
                }
            }
        }
        Ok(body_data.clone())
    }

    fn get_base_table(entity_table: &str) -> String {
        if let Some(base_table) = entity_table.strip_suffix("_ev") {
            return base_table.to_string();
        } else if let Some(base_table) = entity_table.strip_suffix("_v") {
            return base_table.to_string();
        }
        entity_table.to_string()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn to_simple(&self) -> RequestDataSimple {
        let code1 = self.application.code.clone();
        let code = match code1{
            Some(val) => val,
            None => "local".to_string(),
        };
        RequestDataSimple {
            base_path: self.base_path.clone(),
            entity_path: self.entity_path.clone(),
            entity_table: self.entity_table.clone(),
            entity_base_table: self.entity_base_table.clone(),
            application_code: code,
            action_path: "".to_string(),
        }
    }

    // fn to_string_simple(&self) -> String {
    //     let mut app_code = "";
    //     if let Some(code) = &self.application.code {
    //         app_code = code.as_str();
    //     }
    //     let mut result = String::new();
    //     result.push_str("base_path: ");
    //     result.push_str(&self.base_path);
    //     result.push_str(", entity_path: ");
    //     result.push_str(&self.entity_path);
    //     result.push_str(", entity_table: ");
    //     result.push_str(&self.entity_table);
    //     result.push_str(", entity_base_table: ");
    //     result.push_str(&self.entity_base_table);
    //     result.push_str(", url_data: ");
    //     result.push_str(", application_code: ");
    //     result.push_str(app_code);
    //     result
    // }
}

impl ToString for RequestData {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("base_path: ");
        result.push_str(&self.base_path);
        result.push_str(", entity_path: ");
        result.push_str(&self.entity_path);
        result.push_str(", entity_table: ");
        result.push_str(&self.entity_table);
        result.push_str(", entity_base_table: ");
        result.push_str(&self.entity_base_table);
        result.push_str(", url_data: ");
        result.push_str(&format!("{:?}", self.url_data));
        result.push_str(", body_data: ");
        result.push_str(&format!("{:?}", self.body_data));
        result.push_str(", request_type: ");
        result.push_str(&format!("{:?}", self.request_type));
        result.push_str(", application: ");
        result.push_str(&format!("{:?}", self.application));
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn check_get_base_table2() {
        let str_list = vec!["po_header_ev", "po_header_v", "po_header_xv", "po_header"];
        for str in str_list {
            let stripped_value = RequestData::get_base_table(str);
             assert_eq!(stripped_value, "po_header");
        }
    }
}
