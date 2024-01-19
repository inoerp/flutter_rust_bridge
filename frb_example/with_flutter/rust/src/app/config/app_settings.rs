#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub run_init_scripts: bool,
    pub debug_level: u8,
    pub enable_js_api: bool,
    pub show_all_columns: bool,
    pub access_secret: String,
    pub auto_create_users: bool,
    pub default_query_limit: i32,
}
