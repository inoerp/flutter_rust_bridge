#[derive(serde::Deserialize, Debug, Clone)]
pub struct OAuthSettings {
    pub user_name: Vec<String>,
    pub email: Vec<String>,
    pub last_name: Vec<String>,
    pub first_name: Vec<String>,
    pub phone_number: Vec<String>,
    pub roles: Vec<String>,
}
