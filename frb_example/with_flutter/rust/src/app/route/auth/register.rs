use crate::model::common::auth::token::RegisterUserSchema;
use crate::model::common::local::rd_sec_user::RdSecUser;
use crate::model::state::global_state::GlobalState;
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use std::collections::HashMap;

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    gs: web::Data<GlobalState>,
) -> impl Responder {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("username", &body.username);
    let users = RdSecUser::find_by_params(gs.sqlite_pools.get("local"), params).await;

    if let Ok(user_list) = users {
        if !user_list.is_empty() {
            return HttpResponse::Conflict().json(
                serde_json::json!({"status": "fail","message": "User with that email already exists"}),
            );
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    // let query_result = sqlx::query_as!(
    //     User,
    //     "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
    //     body.name.to_string(),
    //     body.email.to_string().to_lowercase(),
    //     hashed_password
    // )
    // .fetch_one(&data.db)
    // .await;

    // match query_result {
    //     Ok(user) => {
    //         let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
    //             "user": filter_user_record(&user)
    //         })});

    //         return HttpResponse::Ok().json(user_response);
    //     }
    //     Err(e) => {
    //         return HttpResponse::InternalServerError()
    //             .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
    //     }
    // }

    HttpResponse::InternalServerError()
        .json(serde_json::json!({"status": "error","message" :"Invalid "}))
}

pub fn get_hashed_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    hashed_password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_hashed_password() {
        let _val = get_hashed_password("admin");
    }
}
