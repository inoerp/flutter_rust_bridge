
use actix_web::{get, HttpResponse};



#[get("/api/config/")]
async fn get_config() -> HttpResponse {
    // Logic to handle GET request for /api/config/ goes here
    HttpResponse::Ok().body("Config data")
}

