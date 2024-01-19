use sqlx::Pool;
use linked_hash_map::LinkedHashMap;
use crate::app::system::error::no_value::NoValueFoundError;



pub trait DbOperation {
    fn get_data_from_sql(
        pool: &Pool<sqlx::MySql>,
        params: Vec<String>,
        sql: &str,
    ) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError>;
}
