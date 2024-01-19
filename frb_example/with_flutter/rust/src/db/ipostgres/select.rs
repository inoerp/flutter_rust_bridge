use linked_hash_map::LinkedHashMap;
use rust_decimal::Decimal;

use sqlx::postgres::{PgColumn, PgRow};
use sqlx::{Column, Pool, Row, TypeInfo};

use crate::app::system::error::no_value::NoValueFoundError;

use super::super::query::QueryData;
use super::IPostgres;

impl IPostgres {
    pub async fn select_using_qd(
        pool: &Pool<sqlx::Postgres>,
        qd: &QueryData,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        Self::select_using_sql(pool, &qd.sql, &qd.params).await
    }

    pub async fn select_using_sql(
        pool: &Pool<sqlx::Postgres>,
        sql: &str,
        params: &Vec<String>,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        let data_rows: Vec<PgRow>;
        if !params.is_empty() {
            let mut x1 = sqlx::query(sql);
            for val in params {
                x1 = x1.bind(val);
            }
            data_rows = x1.fetch_all(pool).await.map_err(|err| {
                NoValueFoundError::new(
                    format!("No records found for sql query 1 . Error {:?}", err).as_str(),
                )
            })?;
        } else {
            data_rows = sqlx::query(sql).fetch_all(pool).await.map_err(|err| {
                NoValueFoundError::new(
                    format!("No records found for sql query 22. Error {:?}", err).as_str(),
                )
            })?;
        }

        let json_data = Self::rows_to_json(data_rows);
        Ok(json_data)
    }

    fn rows_to_json(rows: Vec<PgRow>) -> Vec<LinkedHashMap<String, String>> {
        let mut ret_vec: Vec<LinkedHashMap<String, String>> = Vec::new();
        if rows.is_empty() {
            return ret_vec;
        }
        let columns = rows[0].columns();
        for row in &rows {
            let mapped_row = Self::get_data_from_row(row, columns);
            ret_vec.push(mapped_row);
        }

        ret_vec
    }

    fn get_data_from_row(row: &PgRow, columns: &[PgColumn]) -> LinkedHashMap<String, String> {
        let mut result: LinkedHashMap<String, String> = LinkedHashMap::new();
        for col in columns {
            let clmn_type_name1 = &(*col.type_info().name()).to_string().to_uppercase();
            let clmn_type_name = clmn_type_name1.as_str();

            match clmn_type_name {
                "VARCHAR" | "CHAR" | "CHARACTER" | "TEXT" | "TINYTEXT" | "MEDIUMTEXT"
                | "LONGTEXT" | "JSON" | "ENUM" => {
                    let value = row.try_get::<String, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(String::from(col.name()), val);
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(String::from(col.name()), String::from(""));
                        }
                    }
                }
                "INT" | "INT4" | "NUMERIC" | "INTEGER" | "TINYINT" | "SMALLINT" | "MEDIUMINT"
                | "BIGINT" => {
                    let value = row.try_get::<i32, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(String::from(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(String::from(col.name()), String::from(""));
                        }
                    }
                }
                "FLOAT" | "DOUBLE" | "REAL" => {
                    let value = row.try_get::<f64, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(String::from(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(String::from(col.name()), String::from(""));
                        }
                    }
                }
                "DECIMAL" => {
                    let value = row.try_get::<Decimal, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(String::from(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(String::from(col.name()), String::from(""));
                        }
                    }
                }
                "DATE" | "INTERVAL" | "TIME" | "DATETIME" | "TIMESTAMP" => {
                    let value = row.try_get::<chrono::NaiveDateTime, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(String::from(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(String::from(col.name()), String::from(""));
                        }
                    }
                }
                "BOOL" | "BOOLEAN" => {
                    let value = row.try_get::<bool, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(String::from(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(String::from(col.name()), String::from(""));
                        }
                    }
                }
                _ => {
                    log::error!(
                        "iPostgres Error in get_data_from_row for column {:?} of clmn_type_name {clmn_type_name} : no mapping ",
                        col.name()
                    );
                    result.insert(String::from(col.name()), String::from(""));
                }
            }
        }
        result
    }

    fn print_mapping_error(col: &PgColumn, clmn_type_name: &str, err: sqlx::Error) {
        if !err.to_string().contains("UnexpectedNullError")
            && !err.to_string().contains("unexpected null")
        {
            log::error!(
                "Error in print_mapping_error for column {:?} : clmn_type_name {:?} : {:?} ",
                col.name(),
                clmn_type_name,
                err.to_string()
            );
        }
    }
}
