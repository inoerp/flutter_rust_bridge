use actix_web::{cookie::Cookie, HttpResponse};

#[derive(Debug)]
pub enum RestResponseCode {
    Ok,
    Error,
}

pub struct RestResponse<'a> {
    pub message: &'a str,
    pub response_code: &'a RestResponseCode,
}

impl<'a> RestResponse<'a> {
    pub fn new(message: &'a str, response_code: &'a RestResponseCode) -> Self {
        Self {
            message,
            response_code,
        }
    }

    pub fn get_response(&self, cookie: Option<Cookie>) -> HttpResponse {
        if let Some(cookie) = cookie {
            match self.response_code {
                RestResponseCode::Ok => HttpResponse::Ok()
                    .cookie(cookie)
                    .append_header(("Content-Type", "application/json"))
                    .body(self.message.to_string()),
                RestResponseCode::Error => HttpResponse::Ok()
                    .cookie(cookie)
                    .append_header(("Content-Type", "application/json"))
                    .body(self.message.to_string()),
            }
        } else {
            match self.response_code {
                RestResponseCode::Ok => HttpResponse::Ok()
                    .append_header(("Content-Type", "application/json"))
                    .body(self.message.to_string()),
                RestResponseCode::Error => HttpResponse::Ok()
                    .append_header(("Content-Type", "application/json"))
                    .body(self.message.to_string()),
            }
        }
    }
}
