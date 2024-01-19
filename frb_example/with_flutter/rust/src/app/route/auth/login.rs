use crate::app::api::response::rest_response::{RestResponse, RestResponseCode};
use crate::app::sec::user_session::UserSessionData;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::model::common::auth::token::{LoginUserSchema, TokenClaims};
use crate::model::common::local::rd_sec_user::RdSecUser;
use crate::model::state::global_state::GlobalState;
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    post, web, Error, HttpResponse,
};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::collections::HashMap;

#[post("/auth/login")]
async fn user_login(
    body: web::Json<LoginUserSchema>,
    gs: web::Data<GlobalState>,
) -> Result<HttpResponse, Error> {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("username", &body.username);
    let data = RdSecUser::find_by_params(gs.sqlite_pools.get("local"), params).await;

    match data {
        Ok(user_list) => {
            let user = user_list
                .first()
                .ok_or_else(|| NoValueFoundError::new("No user found"))?;
            let password = user
                .password
                .as_ref()
                .ok_or_else(|| NoValueFoundError::new("No password found"))?;
            let parsed_hash_val = PasswordHash::new(password);

            match parsed_hash_val {
                Ok(parsed_hash) => {
                    let is_valid = Argon2::default()
                        .verify_password(body.password.as_bytes(), &parsed_hash)
                        .map_or(false, |_| true);

                    if !is_valid {
                        return Ok(RestResponse::new(
                            "Invalid user credentials",
                            &RestResponseCode::Ok,
                        )
                        .get_response(None));
                    }
                    let now = Utc::now();
                    let iat = now.timestamp() as usize;
                    let exp = (now + Duration::minutes(480)).timestamp() as usize; //TODO move 480 to config.yaml
                    let user_id = user
                        .id
                        .ok_or_else(|| NoValueFoundError::new("No password found"))?
                        .to_string();
                    let claims: TokenClaims = TokenClaims {
                        sub: user_id,
                        exp,
                        iat,
                    };

                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(gs.settings.jwt_settings.secret.as_ref()),
                    )
                    .map_err(|err| {
                        let msg = format!("Unable to generate token: {}", err);
                        NoValueFoundError::new(&msg)
                    })?;

                    let cookie: Cookie = Cookie::build("token", token.to_owned())
                        .path("/")
                        .max_age(ActixWebDuration::new(60 * 60, 0))
                        .http_only(true)
                        .finish();
                    // let ret_data =
                    //     get_response_body(gs.sqlite_pools.get("local"), &token, user).await;
                    let ud = UserSessionData::init_for_user(gs.sqlite_pools.get("local"), &token, user).await?;  
                    let ret_data = serde_json::json!(ud).to_string();  
                    return Ok(RestResponse::new(&ret_data, &RestResponseCode::Ok)
                        .get_response(Some(cookie)));
                }
                Err(err) => {
                    log::error!("Error in password hashing : {:?}", err);
                    return_error("Error in password hashing")
                }
            }
        }
        Err(err) => {
            log::error!("Invalid user details : {:?}", err);
             return_error("Invalid user details")
        }
    }
}

fn return_error(message: &str) -> Result<HttpResponse, Error> {
    let ret_data = serde_json::json!({"status": "fail", "message": message}).to_string();
    return Ok(RestResponse::new(&ret_data, &RestResponseCode::Ok).get_response(None));
}

