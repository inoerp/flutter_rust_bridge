use linked_hash_map::LinkedHashMap;
use rust_decimal::Decimal;
use sqlx::mysql::{MySqlColumn, MySqlRow};
use sqlx::{Column, Pool, Row, TypeInfo};

use super::super::query::QueryData;
use super::IMySql;
use crate::app::system::error::no_value::NoValueFoundError;
use crate::app::utils::istr::snake_to_camel;

impl IMySql {
    pub async fn select_using_qd(
        pool: &Pool<sqlx::MySql>,
        qd: &QueryData,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        Self::select_using_sql(pool, &qd.sql, &qd.params).await
    }

    pub async fn select_using_sql(
        pool: &Pool<sqlx::MySql>,
        sql: &str,
        params: &Vec<String>,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        let data_rows: Vec<MySqlRow>;
        let mut conn = pool
            .acquire()
            .await
            .map_err(|_err| NoValueFoundError::new("Unable to get db conn from pool"))?;
        if !params.is_empty() {
            let mut x1 = sqlx::query(sql);
            for val in params {
                x1 = x1.bind(val);
            }
            data_rows = x1.fetch_all(&mut conn).await.map_err(|err| {
                NoValueFoundError::new(
                   format!("error_code_imysql_select_1: No records found for sql mysql query  {}. Error  {:?}", sql, err).as_str(),

                )
            })?;
        } else {
            data_rows = sqlx::query(sql).fetch_all(&mut conn).await.map_err(|err| {
                NoValueFoundError::new(
                    format!("error_code_imysql_select_22: No records found for sql mysql query  {}. Error  {:?}", sql, err).as_str(),
                )
            })?;
        }

        let json_data = Self::rows_to_json(data_rows);
        Ok(json_data)
    }

    fn rows_to_json(rows: Vec<MySqlRow>) -> Vec<LinkedHashMap<String, String>> {
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

    fn get_data_from_row(row: &MySqlRow, columns: &[MySqlColumn]) -> LinkedHashMap<String, String> {
        let mut result: LinkedHashMap<String, String> = LinkedHashMap::new();
        for col in columns {
            let clmn_type_name1 = &(*col.type_info().name()).to_string().to_uppercase();
            let clmn_type_name = clmn_type_name1.as_str();

            match clmn_type_name {
                "VARCHAR" | "CHAR" | "TEXT" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" | "JSON"
                | "ENUM" => {
                    let value = row.try_get::<String, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val);
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                "INT" | "NUMERIC" | "INTEGER" | "TINYINT" | "SMALLINT" | "MEDIUMINT" | "BIGINT" => {
                    let value = row.try_get::<i32, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                "INT UNSIGNED" | "TINYINT UNSIGNED" | "SMALLINT UNSIGNED"
                | "MEDIUMINT UNSIGNED" | "BIGINT UNSIGNED" => {
                    let value = row.try_get::<u32, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                "FLOAT" | "DOUBLE" => {
                    let value = row.try_get::<f64, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                "DECIMAL" => {
                    let value = row.try_get::<Decimal, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                "DATE" => {
                    let value = row.try_get::<chrono::NaiveDate, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                "DATETIME" | "TIME" | "TIMESTAMP" => {
                    let value = row.try_get::<chrono::NaiveDateTime, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                "BOOL" | "BOOLEAN" => {
                    let value = row.try_get::<bool, &str>(col.name());
                    match value {
                        Ok(val) => {
                            result.insert(snake_to_camel(col.name()), val.to_string());
                        }
                        Err(err) => {
                            Self::print_mapping_error(col, clmn_type_name, err);
                            result.insert(snake_to_camel(col.name()), String::from(""));
                        }
                    }
                }
                _ => {
                    log::error!(
                        "iMySQL Error in get_data_from_row for column {:?} of clmn_type_name {clmn_type_name} : no mapping ",
                        col.name()
                    );
                    result.insert(snake_to_camel(col.name()), String::from(""));
                }
            }
        }
        result
    }

    fn print_mapping_error(col: &MySqlColumn, clmn_type_name: &str, err: sqlx::Error) {
        if !err.to_string().contains("UnexpectedNullError")
            && !err.to_string().contains("unexpected null")
        {
            log::error!(
                "iMySQL Error in print_mapping_error for column {:?} : clmn_type_name {:?} : {:?} ",
                col.name(),
                clmn_type_name,
                err.to_string()
            );
        }
    }
}

#[cfg(test)]
mod test {
    use crate::db::imysql::IMySql;

    #[tokio::test]
    async fn test_get_data_from_sql() {
        let entity_path = "PoHeaderEv";
        let sql = format!(
            "SELECT * FROM sys_action_ev 
       where 1 = 1 
       AND  vv_path_url ='{entity_path}' ORDER BY sequence ASC "
        );
        let pools = IMySql::get_connection_pools_for_test()
            .await
            .expect("Unable to fetch pool ");
        let pool = pools.get("ierp").expect("Unable to fetch pool ");
        let _data = IMySql::select_using_sql(pool, &sql, &vec![]).await;
    }
}
