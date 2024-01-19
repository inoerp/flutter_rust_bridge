use std::collections::HashMap;

use serde_json::json;
use linked_hash_map::LinkedHashMap;
use crate::app::api::request::model::body_data::BodyData;
use crate::{
    app::api::request::model::request_data::RequestData,
    app::{cache::global_cache::GlobalCache, system::error::no_value::NoValueFoundError},
};

use super::entity::{js_input::JsRustInput, js_output::JsOutput, js_trigger_point::JsTriggerPoint};

#[derive(Debug)]
pub struct JsValidation<'a> {
    base_path: &'a str,
    entity_path: &'a str, //if proceed further is false then return
}

impl<'a> JsValidation<'a> {
    pub fn new(base_path: &'a str, entity_path: &'a str) -> Self {
        Self {
            base_path,
            entity_path,
        }
    }

    pub async fn validate_after(
        &self,
        trigger_point: JsTriggerPoint,
        rd: &RequestData,
        data: &Vec<linked_hash_map::LinkedHashMap<String, serde_json::Value>>,
    ) -> Result<JsOutput, NoValueFoundError> {
        //Check if validation required
        //if not required send not_required js
        //else complete the validation
        let js_obj = GlobalCache::get_js_objects(self.base_path, self.entity_path, &trigger_point);
        log::info!("Js Object is {:?}", js_obj);
        match js_obj {
            Some(val) => {
                log::info!("Js val is {:?}", val);
                let data: JsRustInput = self.get_js_input_for_after_request(rd, data);
                val.run(data)
            }
            None => Ok(JsOutput::not_required()),
        }
    }

    pub async fn validate_before(
        &self,
        trigger_point: JsTriggerPoint,
        rd: &RequestData,
    ) -> Result<JsOutput, NoValueFoundError> {
        //Check if validation required
        //if not required send not_required js
        //else complete the validation
        let js_obj = GlobalCache::get_js_objects(self.base_path, self.entity_path, &trigger_point);
        match js_obj {
            Some(val) => {
                let data: JsRustInput = self.get_js_input_from_request_data(rd);
                val.run(data)
            }
            None => Ok(JsOutput::not_required()),
        }
    }

    fn get_js_input_for_after_request(
        &self,
        rd: &RequestData,
        data2: &Vec<LinkedHashMap<String, serde_json::Value>>,
    ) -> JsRustInput {
        let data_str: serde_json::Value = serde_json::json!(data2);
        let req_data_str = serde_json::json!(&rd);
        let data: JsRustInput =
            JsRustInput::new(data_str, req_data_str, "".to_string(), "".to_string());
        data
    }

    fn get_js_input_from_request_data(&self, rd: &RequestData) -> JsRustInput {
        let mut data2: HashMap<String, serde_json::Value> = HashMap::new();
        data2.insert("entityPath".to_string(), json!(rd.entity_path));
        data2.insert("actionPath".to_string(), json!(rd.action_path));

        match &rd.body_data {
            BodyData::SingleItem(item) => {
                data2.extend(item.iter().map(|(k, v)| (k.to_string(), json!(v))));
            }
            BodyData::MultipleItems(items) => {
                data2.insert("items".to_string(), json!(items));
            }
            _ => {}
        }
        //data2.insert("invTransactionDocHeaderId".to_string(), "1".to_string());
        let data_str: serde_json::Value = serde_json::json!(&data2);
        let req_data_str = serde_json::json!(&rd);
        let data: JsRustInput =
            JsRustInput::new(data_str, req_data_str, "".to_string(), "".to_string());
        data
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{app::api::request::model::request_type::RequestType, startup};
    #[tokio::test]
    async fn test_validation() {
        startup::init().await.expect("Unable to start up");
        let js_val = JsValidation::new("ierp", "AmUnitTest");
        let gs = GlobalCache::get_global_state().expect("unable to find gs");
        let rd: RequestData = RequestData::new(
            &gs,
            "ierp".to_string(),
            "AmUnitTest".to_string(),
            vec![],
            RequestType::Get,
        )
        .expect("unable to find rd ");
        let res = js_val
            .validate_before(JsTriggerPoint::BeforeGet, &rd)
            .await
            .expect("unable to find js result ");
        log::info!("res is {:?}", res);
    }
}
