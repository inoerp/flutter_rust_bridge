use serde;

use crate::app::config::{
    db_settings::DbSettings, email_settings::EmailSettings, oauth_settings::OAuthSettings, app_settings::AppSettings,
};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Settings {
    pub host: String,
    pub protocol: String,
    pub port: String,
    pub cert_file: String,
    pub key_file: String,
    pub app_settings: AppSettings,
    pub jwt_settings: JwtSettings,
    pub db_settings: Vec<DbSettings>,
    pub email_settings: EmailSettings,
    pub oauth_settings: OAuthSettings,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct JwtSettings {
    pub secret: String,
    pub expired_in: String,
    pub max_age: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}
