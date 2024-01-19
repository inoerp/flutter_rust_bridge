use crate::{app::system::error::no_value::NoValueFoundError, };

use super::{ IMySql};
use linked_hash_map::LinkedHashMap;
use sqlx::Pool;

pub async fn get_actions_mysql(
    pool: &Pool<sqlx::MySql>,
    entity_path: &str,
) -> Result<Vec<LinkedHashMap<String, String>>, NoValueFoundError> {
    //let sys_action_opn = GlobalCache::get_action(base_path, action_path).await;
    let sql = format!(
        "SELECT * FROM sys_action_ev 
   where 1 = 1 
   AND  vv_path_url ='{entity_path}' ORDER BY sequence ASC "
    );
    let data: Vec<linked_hash_map::LinkedHashMap<String, String>> =
    IMySql::select_using_sql(pool,&sql, &vec![]).await?;
    Ok(data)
}

#[cfg(test)]
mod test {
    
    use crate::db::imysql::IMySql;

    use super::*;
    #[tokio::test]
    async fn test_get_actions_mysql() {
        let pools = IMySql::get_connection_pools_for_test()
            .await
            .expect("Failed to fetch pool");
        let pool = pools.get("ierp").expect("Failed to fetch pool");
        let _data2 = get_actions_mysql(pool, "PoHeaderEv")
            .await
            .expect("Failed to fetch pool");
    }
}
