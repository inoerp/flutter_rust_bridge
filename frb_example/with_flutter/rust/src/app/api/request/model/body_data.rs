use crate::app::system::error::no_value::NoValueFoundError;
use actix_web::{web, Error};
use serde_json::Value;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyData {
    None,
    SingleItem(HashMap<String, String>),
    MultipleItems(Vec<HashMap<String, String>>),
}

impl BodyData {
    pub fn get_body_data(body: web::BytesMut) -> Result<BodyData, Error> {
        let body_str = std::str::from_utf8(&body).map_err(|err| {
            let msg = format!("Unable to read request body: {}", err);
            NoValueFoundError::new(&msg)
        })?;

        let json_data: Value = serde_json::from_str(body_str).map_err(|err| {
            let msg = format!("Unable to parse body. Send body in JSON format: {}", err);
            NoValueFoundError::new(&msg)
        })?;

        if let Value::Object(obj) = json_data {
            if obj.contains_key("objects") {
                let items = obj.get("objects").ok_or_else(|| {
                    NoValueFoundError::new("Unable to parse objects in body data")
                })?;
                Self::get_multiple_items(items)
            } else {
                Self::get_single_item(Value::Object(obj))
            }
        } else {
            Self::get_single_item(json_data)
        }
    }

    fn get_single_item(json_data: Value) -> Result<BodyData, Error> {
        if let Some(obj) = json_data.as_object() {
            let hashmap_data: HashMap<String, String> = obj
                .iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
                .collect();

            let ret_val = BodyData::SingleItem(hashmap_data);
            Ok(ret_val)
        } else {
            Err(NoValueFoundError::new("Invalid format for single item").into())
        }
    }

    fn get_multiple_items(json_data: &Value) -> Result<BodyData, Error> {
        if let Value::Array(items) = json_data {
            let mut parsed_items = Vec::new();

            for item_data in items {
                if let Value::Object(obj) = item_data {
                    let hashmap_data: HashMap<String, String> = obj
                        .iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
                        .collect();

                    parsed_items.push(hashmap_data);
                }
            }

            let ret_val = BodyData::MultipleItems(parsed_items);
            Ok(ret_val)
        } else {
            Err(NoValueFoundError::new("Invalid format for multiple items").into())
        }
    }
}
