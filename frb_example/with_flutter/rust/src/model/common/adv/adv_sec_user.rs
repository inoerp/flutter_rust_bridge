use std::collections::HashMap;

use sqlx::Pool;

use crate::{app::system::error::no_value::NoValueFoundError, db::isqlite::ISqlite};

pub async fn get_user_role(
    pool: Option<&Pool<sqlx::Sqlite>>,
    user_id: &str,
) -> Result<HashMap<String, String>, NoValueFoundError> {
    let mut role_sql = String::from(
        "
    SELECT ra.obj_class_name, ra.access_level
    FROM rd_sec_role_access ra
    WHERE ra.role_code = 'USER'
    UNION
    SELECT ra.obj_class_name, ra.access_level
    FROM rd_sec_user_role ur,
    rd_sec_role_access ra
    WHERE 1 = 1
    AND (ur.role_code = ra.role_code)
",
    );

    role_sql += &format!(" AND ur.user_id = '{}'", user_id);
    let params: Vec<String> = Vec::new();
    let data = ISqlite::select_using_sql(pool, &role_sql, &params)
        .await
        .map_err(|err| {
            NoValueFoundError::new(format!("Unable to prase user roles {:?}", err).as_str())
        })?;
    let mut mapped_data: HashMap<String, String> = HashMap::new();
    for item in data {
        if let (Some(class), Some(access_level)) =
            (item.get("obj_class_name"), item.get("access_level"))
        {
            mapped_data.insert(class.to_owned(), access_level.to_owned());
        }
    }
    Ok(mapped_data)
}

pub async fn get_user_org_data(
    pool: Option<&Pool<sqlx::Sqlite>>,
    name: &str,
    user_id: &str,
) -> Result<HashMap<String, String>, NoValueFoundError> {
    let role_sql = format!(
        "SELECT lol.lov_value, lol.value_code
    FROM rd_sys_lov_lines lol,
    rd_sec_user user
    WHERE 1 = 1 AND lol.lov_code = user.'{}' 
    AND user.id = '{}' 
    ORDER BY lol.seq ASC ",
        name, user_id
    );

    let params: Vec<String> = Vec::new();
    let data = ISqlite::select_using_sql(pool, &role_sql, &params)
        .await
        .map_err(|err| {
            NoValueFoundError::new(format!("Unable to prase user roles {:?}", err).as_str())
        })?;

    let mut mapped_data: HashMap<String, String> = HashMap::new();
    for item in data {
        if let (Some(value_code), Some(lov_value)) = (item.get("value_code"), item.get("lov_value"))
        {
            mapped_data.insert(value_code.to_owned(), lov_value.to_owned());
        }
    }
    Ok(mapped_data)
}
