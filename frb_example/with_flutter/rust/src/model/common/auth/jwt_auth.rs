use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::model::state::global_state::GlobalState;

use super::token::TokenClaims;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).expect("Unable to convert json to string")
        )
    }
}

pub struct JwtMiddleware {
    pub user_id: i32,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let json_error = ErrorResponse {
            status: "fail".to_string(),
            message: "Invalid token: Please log in to complete the action.".to_string(),
        };

        let data: &web::Data<GlobalState> =
            if let Some(val) = req.app_data::<web::Data<GlobalState>>() {
                val
            } else {
                return ready(Err(ErrorUnauthorized(json_error)));
            };

        let token1 = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .and_then(|h| h.to_str().ok().map(|s| s.split_at(7).1.to_string()))
            })
            .ok_or_else(|| ErrorUnauthorized(json_error));

        let json_error = ErrorResponse {
            status: "fail".to_string(),
            message: "Invalid token: token is missing both in headers and cookies.".to_string(),
        };

        let token = match token1 {
            Ok(val) => val,
            Err(_err) => return ready(Err(ErrorUnauthorized(json_error))),
        };

        let claims = match decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(data.settings.jwt_settings.secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let user_id: i32 = if let Ok(val) = claims.sub.as_str().parse() {
            val
        } else {
            return ready(Err(ErrorUnauthorized(json_error)));
        };

        req.extensions_mut().insert::<i32>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}
