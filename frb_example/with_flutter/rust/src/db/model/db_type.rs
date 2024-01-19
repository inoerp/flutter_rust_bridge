use serde::{self, Serialize, Deserialize};
use std::fmt;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbType {
    MySql,
    Postgres,
    Sqlite,
    MsSql,
    Oracle
}

impl DbType {
    pub fn from_string(s: &str) -> DbType {
        match s.to_lowercase().as_str() {
            "mysql" => DbType::MySql,
            "postgres" => DbType::Postgres,
            "sqlite" => DbType::Sqlite,
            "mssql" => DbType::MsSql,
            "oracle" => DbType::Oracle,
            _ => DbType::Sqlite, // Set Sqlite as the default value
        }
    }

}


impl fmt::Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            DbType::MySql => "mysql",
            DbType::Postgres => "postgres",
            DbType::Sqlite => "sqlite",
            DbType::MsSql => "mssql",
            DbType::Oracle => "oracle",
        };
        write!(f, "{}", output)
    }
}
