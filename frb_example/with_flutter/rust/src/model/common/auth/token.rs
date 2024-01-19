use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

//used to validate incoming data during the account registration process
#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

//used to validate incoming data during user login
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}