use crate::model::common::auth::jwt_auth;
use crate::model::state::global_state::GlobalState;
use crate::{app::api::url::url_data::UrlData, model::data::condition::Condition};
use actix_web::dev::Extensions;
use actix_web::{web, HttpMessage, HttpResponse};

use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::query::QueryData;
use actix_web::get;
use actix_web::{Error, HttpRequest};

use crate::app::api::request::get_request::GetRequest;
use crate::app::api::request::model::request_data::RequestData;

#[get("/api/{basePath}/{entityName}/data")]
async fn get_me_handler(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let mut user_id: i32 = 0;

    {
        let req_extensions: std::cell::Ref<Extensions> = req.extensions();
        if let Some(extension) = req_extensions.get::<i32>() {
            user_id = *extension;
        }
    }

    if user_id == 0 {
        return Ok(RestResponse::new("Invalid user id", &RestResponseCode::Ok).get_response(None));
    }

    let mut rd: RequestData =
        RequestData::init_from_http_request(&req, gs.as_ref(), BodyData::None)?;

    let url_data = UrlData {
        key: "id".to_string(),
        value: user_id.to_string(),
        condition: Condition::Eq,
    };
    rd.url_data.push(url_data);

    let qd: QueryData = QueryData::init_from_request_data(gs.as_ref(), rd);

    let req = GetRequest::new(gs.as_ref(), &qd).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    let data = req.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;

    Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response(None))
}
