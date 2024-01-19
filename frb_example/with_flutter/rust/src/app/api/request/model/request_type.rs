use actix_web::http::Method;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    Get,
    Post,
    Patch,
    Delete,
    Unknown,
}

impl RequestType {
    pub fn get_type_from_method(method: &Method) -> RequestType {
        if method == Method::GET {
            RequestType::Get
        } else if method == Method::POST {
            RequestType::Post
        } else if method == Method::PATCH {
            RequestType::Patch
        } else if method == Method::DELETE {
            RequestType::Delete
        } else {
            RequestType::Unknown
        }
    }
}
