use actix_web::delete;
use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::app::api::request::delete_request::DeleteRequest;
use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::cache::global_cache::GlobalCache;

use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::js::entity::js_trigger_point::JsTriggerPoint;
use crate::app::js::validation::JsValidation;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{action::ActionData, sql_data::SqlActionType};

use crate::app::api::request::action_request::ActionRequest;
use crate::model::common::auth::jwt_auth;
use crate::model::state::global_state::GlobalState;

#[delete("/api/{basePath}/{entityName}")]
async fn delete_entity(
    req: HttpRequest,
    _gs: web::Data<GlobalState>,
    us: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let session_data = GlobalCache::get_user_session_data(us.user_id.to_string().as_str()).ok_or_else(||{
        NoValueFoundError::new("Invalid user session")
     })?;

    let rd: RequestData =
        RequestData::init_from_http_request(&req, _gs.as_ref(),  BodyData::None)?;
    
    // javascript validation
    let js_validation = JsValidation::new(&rd.base_path, &rd.entity_path);
    let val_result = js_validation
        .validate_before(JsTriggerPoint::BeforeDelete, &rd)
        .await?;
    if !val_result.rd_proceed_status {
        return Ok(
            RestResponse::new(&val_result.rd_proceed_message, &RestResponseCode::Ok)
                .get_response(None),
        );
    }

    // get action data
    let action_data: ActionData = ActionData::init(_gs.as_ref(), rd, SqlActionType::Delete, &session_data).await?;

    if action_data.params.is_empty() {
        let msg = "No data to update. Send the data to delete in header";
        return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response(None));
    }

    let request = ActionRequest::new(_gs.as_ref(), &action_data).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    let action_request = DeleteRequest::new(&request);

    let data = action_request.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response(None))
}
