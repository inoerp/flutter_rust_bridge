use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::sql_data::SqlActionType;

use super::super::action::ActionData;

use linked_hash_map::LinkedHashMap;
use sqlx::mysql;
use sqlx::Pool;

pub async fn db_execution(
    pool: &Pool<sqlx::MySql>,
    ad: &ActionData,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    complete_db_execution(ad, pool).await
}

pub async fn db_execution_simple(
    pool: &Pool<sqlx::MySql>,
    sql: &str,
    params: &Vec<String>,
    action_type: SqlActionType,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    let data_rows: mysql::MySqlQueryResult;
    if !params.is_empty() {
        let mut x1 = sqlx::query(sql);
        for val in params {
            x1 = x1.bind(val);
        }
        data_rows = x1.execute(pool).await.map_err(|err| {
            NoValueFoundError::new(
                format!("No records found for sql query 1 . Error {:?}", err).as_str(),
            )
        })?;
    } else {
        data_rows = sqlx::query(sql).execute(pool).await.map_err(|err| {
            NoValueFoundError::new(
                format!("No records found for sql query 1 . Error {:?}", err).as_str(),
            )
        })?;
    }

    let json_data = query_result_to_json(action_type, data_rows);
    Ok(json_data)
}


async fn complete_db_execution(
    ad: &ActionData,
    pool: &Pool<sqlx::MySql>,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    let data_rows: mysql::MySqlQueryResult;
    if !ad.params.is_empty() {
        let mut x1 = sqlx::query(&ad.sql);
        for val in &ad.params {
            x1 = x1.bind(val);
        }
        data_rows = x1
            .execute(pool)
            .await
            .map_err(|_err| NoValueFoundError::new("Unable to get db conn from pool"))?;
    } else {
        data_rows = sqlx::query(&ad.sql)
            .execute(pool)
            .await
            .map_err(|_err| NoValueFoundError::new(format!("Unable to complete db action. Error {:?}", _err).as_str()))?;
    }

    let json_data = query_result_to_json(ad.action_type.clone(), data_rows);
    Ok(json_data)
}

fn query_result_to_json(
    action_type: SqlActionType,
    rows: mysql::MySqlQueryResult,
) -> Vec<LinkedHashMap<String, String>> {
    let mut ret_vec: Vec<LinkedHashMap<String, String>> = Vec::new();

    if action_type == SqlActionType::Update || action_type == SqlActionType::Insert {
        let last_insert_id = rows.last_insert_id().to_string();
        let mut map: LinkedHashMap<String, String> = LinkedHashMap::new();
        map.insert(String::from("last_insert_id"), last_insert_id);
        ret_vec.push(map);

        let rows_affected = rows.rows_affected().to_string();
        let mut map: LinkedHashMap<String, String> = LinkedHashMap::new();
        map.insert(String::from("rows_affected"), rows_affected);
        ret_vec.push(map);
    } else {
        let mut map: LinkedHashMap<String, String> = LinkedHashMap::new();
        map.insert(String::from("result"), "completed".to_string());
        ret_vec.push(map);
    }

    ret_vec
}
