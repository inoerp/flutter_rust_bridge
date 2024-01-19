use actix_web::{error, patch, post, web, Error, HttpRequest, HttpResponse};
use futures::StreamExt;
use linked_hash_map::LinkedHashMap;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::app::api::request::get_request::GetRequest;
use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::api::request::patch_request::PatchRequest;
use crate::app::api::request::post_request::PostRequest;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::api::url::url_data::UrlData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::action::{ActionData, ActionType};
use crate::model::data::condition::Condition;

use crate::app::api::request::action_request::ActionRequest;
use crate::db::query::QueryData;
use crate::model::state::global_state::GlobalState;

const MAX_SIZE: usize = 262_1440; // max payload size is 2560k

#[post("/api/{basePath}/{entityName}")]
async fn post_entity(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
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
    let body_data_str: HashMap<String, String> = get_body_data_str(body.clone())?;
    let body_data = BodyData::get_body_data(body)?;

    let rd: RequestData =
        RequestData::init_from_http_request(&req, gs.as_ref(), body_data_str, body_data);
    // get action data
    let action_data: ActionData =
        ActionData::init(gs.as_ref(), rd.clone(), ActionType::Insert).await;

    if action_data.params.len() < 1 {
        let msg = "No data to update. Send the data to update in body in json format";
        return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response());
    }

    let request = ActionRequest::new(gs.as_ref(), &action_data).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    let action_request = PostRequest::new(&request);

    let data = action_request.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    if data.len() > 0 {
        let mut last_insert_id = 0;
        let mut rows_affected = 0;
        for row in &data {
            if row.contains_key("rows_affected") {
                rows_affected = row
                .get("rows_affected")
                .ok_or_else(|| NoValueFoundError::new("Invalid rows_affected"))?
                .parse()
                .map_err(|err| NoValueFoundError::new("Unable to parse rows_affected"))?;
            } else if row.contains_key("last_insert_id") {
                last_insert_id = row
                .get("last_insert_id")
                .ok_or_else(|| NoValueFoundError::new("Invalid last_insert_id"))?
                .parse()
                .map_err(|err| NoValueFoundError::new("Unable to parse last_insert_id"))?;
            }
        }
        if last_insert_id > 0 && rows_affected > 0 {
            if rows_affected > 1 {
                let ret_str = serde_json::json!(&data).to_string();
                return Ok(RestResponse::new(&ret_str, &RestResponseCode::Ok).get_response());
            } else {
                return new_entity_data(rd, gs.as_ref(), &last_insert_id.to_string()).await;
            }
        } else {
            return Ok(
                RestResponse::new("Requested completed", &RestResponseCode::Ok).get_response(),
            );
        }
    } else {
        return Ok(RestResponse::new("Requested completed", &RestResponseCode::Ok).get_response());
    }

    //get new entity data
    // new_entity_data(rd, _gs.as_ref()).await

    //Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response())

    //Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response())
}

async fn new_entity_data(
    mut rd: RequestData,
    gs: &GlobalState,
    last_insert_id: &String,
) -> Result<HttpResponse, Error> {
    let url_data = UrlData {
        key: "glPaymentTermId".to_string(),
        value: last_insert_id.to_string(),
        condition: Condition::Eq,
    };
    rd.url_data.push(url_data);
    let qd: QueryData = QueryData::init_from_request_data(rd);

    let req = GetRequest::new(gs, &qd).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    let data = req.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response())
}

fn get_body_data_str(body: web::BytesMut) -> Result<HashMap<String, String>, Error> {
    let body_str = std::str::from_utf8(&body).map_err(|err| {
        let msg = "Unable to read request body : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    let json_data: Map<String, Value> = serde_json::from_str(&body_str).map_err(|err| {
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

fn get_body_data_json(body: web::BytesMut) -> Result<Map<String, Value>, Error> {
    let body_str = std::str::from_utf8(&body).map_err(|err| {
        let msg = "Unable to read request body : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    let json_data: Map<String, Value> = serde_json::from_str(&body_str).map_err(|err| {
        let msg = "Unable to parse body. Send body in json format : ".to_string()
            + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    Ok(json_data)
}

#[post("/authenticate")]
async fn authenticate() -> HttpResponse {
    // Logic to handle POST request for /authenticate goes here
    HttpResponse::Ok().body("Authenticated!")
}
