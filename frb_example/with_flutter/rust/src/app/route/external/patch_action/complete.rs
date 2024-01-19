use actix_web::{ web,HttpResponse};
use crate::app::api::request::model::request_data::RequestData;
use crate::app::api::request::patch_request::PatchRequest;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::route::external::patch_action::doc_copy;
use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{action::ActionData, sql_data::SqlActionType};

use crate::app::api::request::action_request::ActionRequest;
use crate::model::entity::patch_action::PatchActionType;
use crate::model::state::global_state::GlobalState;

use super::PatchAction;

impl PatchAction {
    pub async fn complete_patch_action(
        rd: &RequestData ,
        gs: &web::Data<GlobalState>,
        patch_action_type: PatchActionType,
        session_data: &UserSessionData,
    ) -> Result<HttpResponse, NoValueFoundError> {

        // get action data
        let action_data_for_doc_update: ActionData;

        if let PatchActionType::CopyDoc(copy_action) = patch_action_type {
            if let Some(mut copy_action_data) = copy_action {
                copy_action_data.update_from_request_data(rd);
                // action_data =
                //     ActionData::init_for_copy(_gs.as_ref(), rd, SqlActionType::Update, copy_action_data).await?;
                return doc_copy::create_copy(
                    gs.as_ref(),
                    rd,
                    SqlActionType::Update,
                    copy_action_data,
                    session_data,
                )
                .await;
            } else {
                let msg: &str = "Invalid action. Doc status does not exit for this action.";
                return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response(None));
            }
        } else {
            action_data_for_doc_update =
                ActionData::init(gs.as_ref(), rd.to_owned(), SqlActionType::Update, session_data).await?;
        }

        if action_data_for_doc_update.params.is_empty() {
            return Self::complete_standard_actions();
        }

        let request = ActionRequest::new(gs.as_ref(), &action_data_for_doc_update).map_err(|err| {
            let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
            NoValueFoundError::new(&msg)
        })?;

        let patch_request = PatchRequest::new(&request);

        let data = patch_request.complete_request().await.map_err(|err| {
            let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
            NoValueFoundError::new(&msg)
        })?;

        let ret_msg = serde_json::json!(data).to_string();

        Ok(RestResponse::new(&ret_msg, &RestResponseCode::Ok).get_response(None))
    }

    fn complete_standard_actions() -> Result<HttpResponse, NoValueFoundError> {
        let msg = "No data to update. Send the data to update in body in json format";
        return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response(None));
    }
}
