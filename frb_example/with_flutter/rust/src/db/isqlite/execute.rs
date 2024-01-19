use crate::app::system::error::no_value::NoValueFoundError;
use crate::db::{action::ActionData, sql_data::SqlActionType};


use linked_hash_map::LinkedHashMap;
use sqlx::sqlite;
use sqlx::Pool;

pub async fn db_execution(
    pool: &Pool<sqlx::Sqlite>,
    ad: &ActionData,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    complete_db_execution(ad, pool).await
}

async fn complete_db_execution(
    ad: &ActionData,
    pool: &Pool<sqlx::Sqlite>,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    let data_rows: sqlite::SqliteQueryResult;
    if !ad.params.is_empty() {
        let mut x1 = sqlx::query(&ad.sql);
        for val in &ad.params {
            x1 = x1.bind(val);
        }
        data_rows = x1.execute(pool).await.map_err(|err| {
            NoValueFoundError::new(
                format!("No records found for sqlite sql query. Error {:?}", err).as_str(),
            )
        })?;
    } else {
        data_rows = sqlx::query(&ad.sql).execute(pool).await.map_err(|err| {
            NoValueFoundError::new(
                format!("No records found for sqlite sql query 2. Error {:?}", err).as_str(),
            )
        })?;
    }


    let json_data = query_result_to_json(ad, data_rows);
    Ok(json_data)
}

fn query_result_to_json( ad: &ActionData, rows: sqlite::SqliteQueryResult) -> Vec<LinkedHashMap<String, String>> {
    let mut ret_vec: Vec<LinkedHashMap<String, String>> = Vec::new();

    if ad.action_type == SqlActionType::Update || ad.action_type == SqlActionType::Insert {
        let last_insert_id = rows.last_insert_rowid().to_string();
        let mut map: LinkedHashMap<String, String> = LinkedHashMap::new();
        map.insert(String::from("last_insert_id"), last_insert_id);
        ret_vec.push(map);
    
        let rows_affected = rows.rows_affected().to_string();
        let mut map: LinkedHashMap<String, String> = LinkedHashMap::new();
        map.insert(String::from("rows_affected"), rows_affected);
        ret_vec.push(map);
    }else{
        let mut map: LinkedHashMap<String, String> = LinkedHashMap::new();
        map.insert(String::from("result"), "completed".to_string());
        ret_vec.push(map);
    }


    ret_vec
}
