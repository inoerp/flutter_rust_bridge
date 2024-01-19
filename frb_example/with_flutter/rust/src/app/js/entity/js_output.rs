use serde::Deserialize;
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct JsOutput {
    pub rd_proceed_status: bool,
    pub rd_proceed_message: String,
    // if yes,  rd_proceed_message will contain data that would be used in UI
    //available for backward compatibility 
    pub rd_data_contains_item: Option<bool>, 
}

impl JsOutput {
    pub fn new(rd_proceed_status: bool,  rd_proceed_message: String, rd_data_contains_item: bool,) -> Self {
        Self {
            rd_proceed_status,
            rd_proceed_message,
            rd_data_contains_item: Some(rd_data_contains_item)
        }
    }

    pub fn not_required() -> Self {
        Self {
            rd_proceed_status: true,
            rd_proceed_message: "".to_string(),
            rd_data_contains_item: Some(false),
        }
    }
    
}
