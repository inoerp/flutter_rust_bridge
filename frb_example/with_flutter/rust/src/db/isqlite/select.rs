use crate::app::system::error::no_value::NoValueFoundError;

use super::super::query::QueryData;
use super::{ISqlite, DB_URL};
use linked_hash_map::LinkedHashMap;
use sqlx::sqlite::{SqliteColumn, SqliteRow};
use sqlx::{Column, Pool, Row, SqlitePool, TypeInfo};

// async fn get_data(qd: QueryData) -> Result<String, Box<dyn Error>> {
//     let pool = SqlitePool::connect(DB_URL).await?;
//     let ret = get_data_from_db(&pool, &qd).await;
//     Ok(String::from(ret))
// }

// async fn get_struct(qd: QueryData) -> Result<Vec<RikdataApplication>, Box<dyn Error>> {
//     let pool = SqlitePool::connect(DB_URL).await?;
//     let ret = get_struct_from_sql_using_pool( qd, &pool,).await?;
//     Ok(ret)
// }

impl ISqlite {
    pub async fn select_using_qd(
        pool: &Pool<sqlx::Sqlite>,
        qd: &QueryData,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        Self::select_using_sql(Some(pool), &qd.sql, &qd.params).await
    }

    pub async fn select_using_sql(
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        sql: &str,
        params: &Vec<String>,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
        let data_rows: Vec<SqliteRow>;
        let mut connection;

        if let Some(pool) = sqlite_pool {
            connection = pool.acquire().await.map_err(|err| {
                NoValueFoundError::new(format!("No sqlite pool found. Error :{:?} ", err).as_str())
            })?;
        } else {
            let pool: sqlx::Pool<sqlx::Sqlite> =
                SqlitePool::connect(DB_URL).await.map_err(|err| {
                    NoValueFoundError::new(
                        format!("Unable to find sqlite pool. Error {:?}", err).as_str(),
                    )
                })?;
            connection = pool.acquire().await.map_err(|err| {
                NoValueFoundError::new(
                    format!("Unable to find sqlite pool. Error {:?}", err).as_str(),
                )
            })?;
        }

        if !params.is_empty() {
            let mut x1 = sqlx::query(sql);
            for val in params {
                x1 = x1.bind(val);
            }
            data_rows = x1.fetch_all(&mut connection).await.map_err(|err| {
                NoValueFoundError::new(format!("Unable to fetch data. Error {:?}", err).as_str())
            })?;
        } else {
            data_rows = sqlx::query(sql)
                .fetch_all(&mut connection)
                .await
                .map_err(|err| {
                    NoValueFoundError::new(
                        format!("Unable to fetch data. Error {:?}", err).as_str(),
                    )
                })?;
        }

        let json_data = Self::rows_to_json(data_rows);

        Ok(json_data)
    }

    fn rows_to_json(rows: Vec<SqliteRow>) -> Vec<LinkedHashMap<String, String>> {
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

    fn get_data_from_row(
        row: &SqliteRow,
        columns: &[SqliteColumn],
    ) -> LinkedHashMap<String, String> {
        let mut result: LinkedHashMap<String, String> = LinkedHashMap::new();
        for col in columns {
            let clmn_type_name = col.type_info().name();
            match clmn_type_name {
                "VARCHAR" | "CHAR" | "TEXT" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" | "JSON"
                | "ENUM" => {
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
                "INT" | "TINYINT" | "SMALLINT" | "MEDIUMINT" | "BIGINT" | "INTEGER" => {
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
                "FLOAT" | "DOUBLE" | "DECIMAL" => {
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
                "DATE" | "TIME" | "DATETIME" | "TIMESTAMP" => {
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
                    let name = col.name();
                    match name {
                        "columnName" | "isPrimary" | "IS_VIEW" | "moduleName" | "columnType"
                        | "columnName " => {
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
                        "isNotNullable" | "columnKey" | "primaryKey" => {
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
                        _ => {
                            log::error!(
                            "Error in get_data_from_row for column {:?} of clmn_type {clmn_type_name} : no mapping ",
                            col.name()
                        );
                            result.insert(String::from(col.name()), String::from(""));
                        }
                    }
                }
            }
        }
        result
    }

    fn print_mapping_error(col: &SqliteColumn, clmn_type_name: &str, err: sqlx::Error) {
        if !err.to_string().contains("UnexpectedNullError") {
            log::error!(
            "iPostgres Error in get_data_from_row for column {:?} : clmn_type_name {:?} : {:?} ",
            col.name(),
            clmn_type_name,
            err.to_string()
        );
        }
    }
}
// #[tokio::test]
// async fn test_struct() {
//     let query = String::from(SQL_APP);
//     let params = Vec::new();
//     let app = String::from("local");
//     let entity = String::from("local");
//     let rd: RequestData = RequestData::new("base_path".to_string(), "entity_path".to_string(), vec![], RequestType::Get);
//     let qd: QueryData = QueryData::new(rd, query, params);
//     let data = get_struct(qd).await;
// }

// #[tokio::test]
// async fn test_connection() {
//     let query = String::from(SQL_APP);
//     let mut params = Vec::new();
//     let app = String::from("local");
//     let entity = String::from("local");
//     let rd: RequestData = RequestData::new("base_path".to_string(), "entity_path".to_string(), vec![], RequestType::Get);
//     let qd: QueryData = QueryData::new(rd, query, params);
//     let data = get_data(qd).await;
// }

// const SQL_APP: &str = "SELECT * from rikdata_application";

// const SQL: &str = " SELECT m.type  TABLE_TYPE,
// CASE
//   WHEN m.type = 'table' THEN 'false'
//   ELSE 'true'
// END IS_VIEW,
//     CASE
//   WHEN p.pk = 1 THEN '1'
//   ELSE '0'
// END isPrimary,
//   m.tbl_name restTableName,
//   p.name  columnName,
//   p.[notnull]  isNotNullable,
//   p.type  columnType,
//   p.pk columnKey,
//   p.pk  primaryKey,
//   'LOCAL_ADMIN' as moduleName,
//   s.nav_group as navGroup,
//   c.select_list as selectList,
//   c.field_group as fieldGroup,
//      c.input_type as inputType
// FROM sqlite_master m
// left outer join pragma_table_info((m.name)) p on m.name <> p.name
// left outer join sys_table_info s on s.base_table_name = m.tbl_name
// left outer join sys_column_info c on c.table_name = m.tbl_name and c.column_name =  p.name
// WHERE m.name NOT LIKE 'sqlite_%'
// AND  p.name != '' and   p.name is not null
// AND m.tbl_name is not null and m.tbl_name !=''
// AND m.tbl_name = 'rikdata_application'";
