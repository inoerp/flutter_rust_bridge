use crate::app::api::request::model::body_data::BodyData;
use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::cache::global_cache::GlobalCache;
use crate::app::js::entity::js_trigger_point::JsTriggerPoint;
use crate::app::js::validation::JsValidation;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::query::QueryData;
use crate::model::state::global_state::GlobalState;
use actix_web::get;
use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::app::api::request::get_request::GetRequest;
use crate::app::api::request::model::request_data::RequestData;
use crate::model::common::auth::jwt_auth;
use actix_files::NamedFile;
use std::path::PathBuf;

// #[get("/")]
pub async fn home_page(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./assets/static/index.html".parse()?;
    let ret = NamedFile::open(path)?;
    print!("serving from {:?}", ret);
    Ok(ret)
}
//all requests contains request data
//get requests contains query data and post/patch/delete requests contains action data
#[get("/api/{basePath}/{entityName}")]
async fn get_entity(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
    us: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
       //TODO validate user access
    let _session_data = GlobalCache::get_user_session_data(us.user_id.to_string().as_str())
        .ok_or_else(|| NoValueFoundError::new("Invalid user session"))?;

    let rd: RequestData = RequestData::init_from_http_request(&req, gs.as_ref(), BodyData::None)?;

    // BeforeGet javascript validation
    // let js_validation = JsValidation::new(&rd.base_path, &rd.entity_path);
    // let val_result = js_validation
    //     .validate_before(JsTriggerPoint::BeforeGet, &rd)
    //     .await?;
    // if !val_result.rd_proceed_status {
    //     return Ok(
    //         RestResponse::new(&val_result.rd_proceed_message, &RestResponseCode::Ok)
    //             .get_response(None),
    //     );
    // }

    let qd: QueryData = QueryData::init_from_request_data(gs.as_ref(), rd);
    let req = GetRequest::new(gs.as_ref(), &qd).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    //download file
    if qd.request_data.entity_path.eq_ignore_ascii_case("download") {
        let res = req.download().await?;
        return Ok(res);
    }

    //return Ok(RestResponse::new("req completed", &RestResponseCode::Ok).get_response(None));

    let data = req.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response(None))
}

#[get("/api/config/{entityName}")]
async fn get_config(req: HttpRequest, gs: web::Data<GlobalState>) -> Result<HttpResponse, Error> {
    
    let rd: RequestData =
        RequestData::init_from_http_request_for_config(&req, gs.as_ref(), BodyData::None)?;
    // BeforeGet javascript validation
    let js_validation = JsValidation::new(&rd.base_path, &rd.entity_path);
    let val_result = js_validation
        .validate_before(JsTriggerPoint::BeforeGet, &rd)
        .await?;

    if !val_result.rd_proceed_status {
        return Ok(
            RestResponse::new(&val_result.rd_proceed_message, &RestResponseCode::Ok)
                .get_response(None),
        );
    }

    let qd: QueryData = QueryData::init_from_request_data(gs.as_ref(), rd);
    let req = GetRequest::new(gs.as_ref(), &qd).map_err(|err| {
        let msg = "Unable to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    //download file
    if qd.request_data.entity_path.eq_ignore_ascii_case("download") {
        let res = req.download().await?;
        return Ok(res);
    }

    let data = req.complete_request().await.map_err(|err| {
        let msg = "Failed to complete the request : ".to_string() + err.to_string().as_str();
        NoValueFoundError::new(&msg)
    })?;
    Ok(RestResponse::new(&data, &RestResponseCode::Ok).get_response(None))
}

#[cfg(test)]
mod tests {
    use url::Url;
    use urlencoding::decode;

    #[test]
    fn test_url() {
        let mut url = Url::parse("http://localhost:8085/api/ierp/download?q=filePath='files%5Cupload'&fileName%21%3D'base-example.xml'&refTableName='sys_file_reference'&refKeyName='sys_file_reference_id'&refKeyValue='203'")
        .expect("wrong url");

        let params: Vec<(String, String)> = url
            .query_pairs()
            .map(|(name, value)| (name.into_owned(), value.into_owned()))
            .collect();
        url.query_pairs_mut().clear().extend_pairs(&params);
        //if !params.is_empty() && params.first().

        let _dec = decode("'files%5Cupload'").expect("wrong url");
    }
}
