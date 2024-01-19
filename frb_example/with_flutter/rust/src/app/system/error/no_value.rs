use std::io;

use actix_web::error::ResponseError;
use actix_web::HttpResponse;

#[derive(Debug, Clone)]
pub struct NoValueFoundError {
    attr_name: String,
}

impl NoValueFoundError {
    pub fn new(attr_name: &str) -> Self {
        Self {
            attr_name: attr_name.to_string(),
        }
    }
}

impl From<NoValueFoundError> for io::Error {
    fn from(err: NoValueFoundError) -> Self {
        // Convert the error into an io::Error instance
        // and return it
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl std::fmt::Display for NoValueFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No value found for {}", self.attr_name)
    }
}

impl std::error::Error for NoValueFoundError {}

impl ResponseError for NoValueFoundError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest()
            .body(format!("Bad request: {}", self.attr_name))
    }
}


