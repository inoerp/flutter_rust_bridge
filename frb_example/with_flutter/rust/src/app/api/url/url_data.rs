use crate::model::data::condition;
use serde::{Deserialize, Serialize};
use urlencoding::decode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlData {
    pub key: String,
    pub value: String,
    pub condition: condition::Condition,
}

impl UrlData {
    pub fn new(str: &str) -> Self {
        let cond = condition::Condition::get(str);
        let cond_str = cond.to_string();
        let mut splitted_val: Vec<&str> = str.split(&cond_str).collect();

           //validate for string conditions such as STARTSWITH or ENDSWITH
        if splitted_val.len() == 1 {
            let str_conditions: Vec<condition::Condition> =
                condition::Condition::get_string_conditions();
            if str_conditions.contains(&cond) {
                if str.contains(&cond_str.to_uppercase()) {
                    splitted_val = str.split(&cond_str.to_uppercase()).collect();
                } else if str.contains(&cond_str.to_lowercase()) {
                    splitted_val = str.split(&cond_str.to_lowercase()).collect();
                }
            }
        }

        let key = match splitted_val.first() {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };
        let value = match splitted_val.get(1) {
            Some(val) => {
                let decoded = decode(val);
                match decoded {
                    Ok(val1) => val1.to_string().replace('\'', ""),
                    Err(err) => {
                        log::error!("Unable to decode url parameter {}. Error : {:?}", val, err);
                        "".to_string()
                    }
                }
            }
            None => "".to_string(),
        };
        Self {
            key,
            value,
            condition: cond,
        }
    }
}

pub fn get_params_from_str(input_str: &str) -> Vec<UrlData> {
    if let Some(val) = input_str.strip_prefix("q="){
        val.split('&').map(UrlData::new).collect()
    }else{
        input_str.split('&').map(UrlData::new).collect()
    }
}

#[test]
fn test_params() {
    let q = "emailstartswithabc@def.com";
    let _url_data = get_params_from_str(q);
}
