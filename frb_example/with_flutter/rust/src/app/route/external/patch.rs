use actix_web::patch;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use futures::StreamExt;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::api::request::patch_request::PatchRequest;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::cache::global_cache::GlobalCache;
use crate::app::route::external::patch_action::patch_sys_action::PatchSysAction;
use crate::app::route::external::patch_action::PatchAction;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{action::ActionData, sql_data::SqlActionType};

use crate::app::api::request::action_request::ActionRequest;
use crate::model::common::auth::jwt_auth;
use crate::model::entity::patch_action::PatchActionType;
use crate::model::state::global_state::GlobalState;

use crate::app::js::entity::js_trigger_point::JsTriggerPoint;
use crate::app::js::validation::JsValidation;

const MAX_SIZE: usize = 262_1440; // max payload size is 2560k

#[patch("/api/{basePath}/{entityName}")]
async fn patch_entity(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
    us: jwt_auth::JwtMiddleware,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut body: web::BytesMut = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let body_data_map: HashMap<String, String> = get_body_data(body)?;
    if body_data_map.is_empty() {
        return Ok(RestResponse::new(
            "No data: Send data to be updated in json format in the body of the request",
            &RestResponseCode::Ok,
        )
        .get_response(None));
    }

    let body_data = BodyData::SingleItem(body_data_map);

    let session_data = GlobalCache::get_user_session_data(us.user_id.to_string().as_str())
        .ok_or_else(|| NoValueFoundError::new("Invalid user session"))?;
    let rd: RequestData = RequestData::init_from_http_request(&req, gs.as_ref(), body_data)?;

    // javascript validation
    let js_validation = JsValidation::new(&rd.base_path, &rd.entity_path);
    let val_result = js_validation
        .validate_before(JsTriggerPoint::BeforePost, &rd)
        .await?;
    if !val_result.rd_proceed_status {
        return Ok(
            RestResponse::new(&val_result.rd_proceed_message, &RestResponseCode::Ok)
                .get_response(None),
        );
    }

    // get action data
    let action_data: ActionData =
        ActionData::init(gs.as_ref(), rd, SqlActionType::Update, &session_data).await?;

    if action_data.params.is_empty() {
        let msg = "No data to update. Send the data to update in body in json format";
        return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response(None));
    }

    let request = ActionRequest::new(gs.as_ref(), &action_data).map_err(|err| {
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

//Ex:    /api/ierp/SdSoHeaderEv(sdSoHeaderId=234)/confirm
#[patch("/api/{basePath}/{entityName}/{actionPath}")]
async fn patch_action(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
    us: jwt_auth::JwtMiddleware,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut body: web::BytesMut = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let mut body_data_map: HashMap<String, String> = get_body_data(body)?;

    let session_data = GlobalCache::get_user_session_data(us.user_id.to_string().as_str())
        .ok_or_else(|| NoValueFoundError::new("Invalid user session"))?;

    let base_path = req
        .match_info()
        .get("basePath")
        .ok_or_else(|| NoValueFoundError::new("Invalid basePath"))?;
    let action_path = req
        .match_info()
        .get("actionPath")
        .ok_or_else(|| NoValueFoundError::new("Invalid action path"))?;
    let action_type: PatchActionType = PatchActionType::from_string(action_path);
    let action_type_clone = action_type.clone();

    if let PatchActionType::DocStatusAction(action) = action_type.clone() {
        if let Some(action) = action {
            body_data_map.insert(
                "doc_status".to_string(),
                action.get_doc_status().to_string(),
            );
        } else {
            let msg: &str = "Invalid action. Doc status does not exit for this action.";
            return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response(None));
        }
    }

    let body_data = BodyData::SingleItem(body_data_map.clone());
    let rd: RequestData = RequestData::init_from_http_request_for_action(
        &req,
        &gs,
        body_data,
        action_type_clone.clone(),
    )?;

    if let PatchActionType::CopyDoc(_copy_action) = action_type {
        let sys_action_opn = GlobalCache::get_patch_action(base_path, action_path).await;
        if let Some(sys_action) = sys_action_opn {
            let ret_val =
                PatchSysAction::complete_sys_action(&rd, &gs, &session_data, &sys_action).await?;
            return Ok(ret_val);
        } else {
            let ret_val =
                PatchAction::complete_patch_action(&rd, &gs, action_type_clone, &session_data)
                    .await?;
            return Ok(ret_val);
        }
    } else if action_type != PatchActionType::None {
        //check if system path
        let sys_action_opn = GlobalCache::get_patch_action(base_path, action_path).await;
        if let Some(sys_action) = sys_action_opn {
            let ret_val =
                PatchSysAction::complete_sys_action(&rd, &gs, &session_data, &sys_action).await?;
            return Ok(ret_val);
        } else {
            let msg: &str = "Action successfully completed";
            return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response(None));
        }
    }
    if body_data_map.is_empty() {
        return Ok(RestResponse::new(
            "No data: Send data to be updated in json format in the body of the request",
            &RestResponseCode::Ok,
        )
        .get_response(None));
    }

    let ret_val =
        PatchAction::complete_patch_action(&rd, &gs, action_type_clone, &session_data).await?;
     Ok(ret_val)
}

fn get_body_data(body: web::BytesMut) -> Result<HashMap<String, String>, Error> {
    let body_str = std::str::from_utf8(&body).map_err(|err| {
        let msg = "Unable to read request body : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    if body_str.is_empty() {
        return Ok(HashMap::new());
    }
    let json_data: Map<String, Value> = serde_json::from_str(body_str).map_err(|err| {
        let msg = "Unable to parse body. Send body in json format : ".to_string()
            + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    let hashmap_data: HashMap<String, String> = json_data
        .iter()
        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
        .collect();
    Ok(hashmap_data)
}
