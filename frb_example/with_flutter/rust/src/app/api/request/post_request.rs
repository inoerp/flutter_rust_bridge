use super::action_request::ActionRequest;
use crate::app::js::entity::js_trigger_point::JsTriggerPoint;
use crate::app::js::validation::JsValidation;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::db_execution::DbExecution;
use crate::model::data::conversion::DataConversion;
use linked_hash_map::LinkedHashMap;


#[derive(Debug, Clone)]
pub struct PostRequest<'a> {
    action_request: &'a ActionRequest<'a>,
}

impl<'a> PostRequest<'a> {
    pub fn new(action_request: &'a ActionRequest<'a>) -> Self {
        Self { action_request }
    }

    pub async fn complete_request(
        &self,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        let request_data = if let Some(request_data) = &self.action_request.ad.request_data {
            request_data
        } else {
            return Err(NoValueFoundError::new(
                "Base path is missing for the request",
            ));
        };
        if let Some(conn_mapping) = self
            .action_request
            .gs
            .conn_pools
            .lock()
            .await
            .get(&request_data.base_path)
        {
            let db_data: Vec<LinkedHashMap<String, String>> =
                DbExecution::execute(conn_mapping, self.action_request.ad).await?;

            //complete js validation after post
            let js_validation = JsValidation::new(
                &request_data.base_path,
                &request_data.entity_path,
            );
            let data2 = DataConversion::linked_map_to_json(&db_data);
            let val_result = js_validation
                .validate_after(
                    JsTriggerPoint::AfterPost,
                    request_data,
                    &data2,
                )
                .await?;
            if !val_result.rd_proceed_status {
                return Ok(get_js_out_put_message(
                    val_result.rd_proceed_message.as_str(),
                ));
            }

            //let ret_str = serde_json::json!(db_data).to_string();
            Ok(db_data)
        } else {
            Ok(get_err_message())
        }
    }
}

fn get_err_message() -> Vec<LinkedHashMap<String, String>> {
    let mut err = LinkedHashMap::new();
    err.insert(
        "error".to_string(),
        "Invalid database connection".to_string(),
    );
    vec![err]
}

fn get_js_out_put_message(msg: &str) -> Vec<LinkedHashMap<String, String>> {
    let mut ret_message = LinkedHashMap::new();
    ret_message.insert(
        "js_message".to_string(),
        msg.to_string(),
    );
    vec![ret_message]
}
