#[derive(serde::Deserialize, Debug, Clone)]
pub struct DbSettings {
    pub name: String,
    pub db_type: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}


impl DbSettings {

    pub fn connection_string_mysql(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_postgres(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_oneapp(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string2(&self) -> String {
        format!(
            "host={} user={} password={}  port={} dbname={}",
            self.host, self.username, self.password, self.port, self.database_name
        )
    }
}
