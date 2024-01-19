use actix_web::{error, post, web, Error, HttpRequest, HttpResponse};
use futures::StreamExt;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::app::api::request::get_request::GetRequest;
use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::request::model::request_data::RequestData;
use crate::app::api::request::post_request::PostRequest;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::api::url::url_data::UrlData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{action::ActionData, sql_data::SqlActionType};
use crate::model::common::adv::adv_menu_path::AdvMenuPath;
use crate::model::common::auth::jwt_auth;
use crate::model::data::condition::Condition;

use crate::app::api::request::action_request::ActionRequest;
use crate::app::cache::global_cache::{self, GlobalCache};
use crate::db::query::QueryData;
use crate::model::state::global_state::GlobalState;

use crate::app::js::entity::js_trigger_point::JsTriggerPoint;
use crate::app::js::validation::JsValidation;

use crate::app::utils::istr as str_utils;
const MAX_SIZE: usize = 262_1440; // max payload size is 2560k

#[post("/api/{basePath}/{entityName}")]
async fn post_entity(
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
    let body_data_map: HashMap<String, String> = get_body_data_map(body.clone())?;
    let body_data = BodyData::get_body_data(body)?;

     if let BodyData::None = body_data{
        if body_data_map.is_empty() {
            return Ok(RestResponse::new(
                "No data: Send data to be updated in json format in the body of the request",
                &RestResponseCode::Ok,
            )
            .get_response(None));
        }
     }
    

    let session_data = GlobalCache::get_user_session_data(us.user_id.to_string().as_str())
        .ok_or_else(|| NoValueFoundError::new("Invalid user session"))?;

    let rd: RequestData =
        RequestData::init_from_http_request(&req, gs.as_ref(),  body_data)?;

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
        ActionData::init(gs.as_ref(), rd.clone(), SqlActionType::Insert, &session_data).await?;

    if action_data.params.is_empty() {
        let msg = "No data to update. Send the data to update in body in json format";
        return Ok(RestResponse::new(msg, &RestResponseCode::Ok).get_response(None));
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

    if !data.is_empty() {
        let mut last_insert_id = 0;
        let mut rows_affected = 0;
        for row in &data {
            if row.contains_key("rows_affected") {
                rows_affected = row
                    .get("rows_affected")
                    .ok_or_else(|| NoValueFoundError::new("Invalid rows_affected"))?
                    .parse()
                    .map_err(|_err| NoValueFoundError::new("Unable to parse rows_affected"))?;
            } else if row.contains_key("last_insert_id") {
                last_insert_id = row
                    .get("last_insert_id")
                    .ok_or_else(|| NoValueFoundError::new("Invalid last_insert_id"))?
                    .parse()
                    .map_err(|_err| NoValueFoundError::new("Unable to parse last_insert_id"))?;
            } else if row.contains_key("last_insert_rowid") {
                //condition for sqlite
                last_insert_id = row
                    .get("last_insert_rowid")
                    .ok_or_else(|| NoValueFoundError::new("Invalid last_insert_rowid"))?
                    .parse()
                    .map_err(|_err| NoValueFoundError::new("Unable to parse last_insert_rowid"))?;
            }
        }
        if last_insert_id > 0 && rows_affected > 0 {
            if rows_affected > 1 {
                let ret_str = serde_json::json!(&data).to_string();
                return Ok(RestResponse::new(&ret_str, &RestResponseCode::Ok).get_response(None));
            } else {
                new_entity_data(rd, gs.as_ref(), &last_insert_id.to_string()).await
            }
        } else {
             Ok(
                RestResponse::new("Requested completed", &RestResponseCode::Ok).get_response(None),
            )
        }
    } else {
         Ok(
            RestResponse::new("Requested completed", &RestResponseCode::Ok).get_response(None),
        )
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
    let p_key_name = get_primary_key_name(&rd, gs).await?;
    rd.url_data = Vec::new();
    let url_data = UrlData {
        key: p_key_name,
        value: last_insert_id.to_string(),
        condition: Condition::Eq,
    };
    rd.url_data.push(url_data);
    let qd: QueryData = QueryData::init_from_request_data(gs, rd);

    let req = GetRequest::new(gs, &qd).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    let data = req.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response(None))
}

async fn get_primary_key_name(
    rd: &RequestData,
    gs: &GlobalState,
) -> Result<String, Box<dyn std::error::Error>> {
    let sqlite_pool = gs.sqlite_pools.get("local");
    let app_code = rd
        .application
        .code
        .as_ref()
        .ok_or_else(|| NoValueFoundError::new("Invalid application code"))?;
    let menu: AdvMenuPath =
        global_cache::get_menu(&rd.base_path, app_code, &rd.entity_path, sqlite_pool).await?;
    let key_fields1 = menu.key_fields.as_ref();
    if let Some(key_fields1) = key_fields1 {
        if !key_fields1.is_empty() {
            let first_field = key_fields1
                .first()
                .ok_or_else(|| NoValueFoundError::new("Invalid MenuFormField"))?;
            if let Some(column_name) = first_field.db_column_name.as_ref() {
                Ok(column_name.to_string())
            } else {
                let field_name = first_field
                    .name
                    .as_ref()
                    .ok_or_else(|| NoValueFoundError::new("Invalid field name"))?;
                let f_name = str_utils::camel_to_snake(field_name);
                Ok(f_name)
            }
        } else {
            Err(Box::new(NoValueFoundError::new("No primary key found")))
        }
    } else {
        Err(Box::new(NoValueFoundError::new("No primary key found")))
    }
}

fn get_body_data_map(body: web::BytesMut) -> Result<HashMap<String, String>, Error> {
    let body_str = std::str::from_utf8(&body).map_err(|err| {
        let msg = "Unable to read request body : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
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

// fn get_body_data_json(body: web::BytesMut) -> Result<Map<String, Value>, Error> {
//     let body_str = std::str::from_utf8(&body).map_err(|err| {
//         let msg = "Unable to read request body : ".to_string() + err.to_string().as_str();
//         NoValueFoundError::new(&msg)
//     })?;
//     let json_data: Map<String, Value> = serde_json::from_str(&body_str).map_err(|err| {
//         let msg = "Unable to parse body. Send body in json format : ".to_string()
//             + err.to_string().as_str();
//         NoValueFoundError::new(&msg)
//     })?;
//     Ok(json_data)
// }
