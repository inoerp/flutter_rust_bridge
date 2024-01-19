#[derive(serde::Deserialize, Debug, Clone)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u32,
    pub smtp_user: String,
    pub smtp_password: String,
    pub smtp_ssl: bool,
    pub smtp_tls: bool,
    pub smtp_from: String,
    pub smtp_from_name: String,
    pub smtp_reply_to: String,
    pub smtp_reply_to_name: String,
}
