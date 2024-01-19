use sqlx::Pool;
use std::collections::HashMap;

use crate::{
    app::system::error::no_value::NoValueFoundError,
    model::common::local::{menu_form_field::MenuFormField, menu_path::MenuPath},
};
#[derive(Debug, Clone)]
pub struct AdvMenuPath {
    pub application_code: String,
    pub menu_path_code: String,
    pub menu_path: Option<MenuPath>,
    pub child_menu_paths: Option<Vec<MenuPath>>,
    pub links: Option<Vec<String>>,
    pub fields: Option<Vec<MenuFormField>>,
    pub key_fields: Option<Vec<MenuFormField>>,
    pub session_fields: Option<Vec<MenuFormField>>,
}

impl AdvMenuPath {
    pub async fn new(
        application_code: String,
        menu_path_code: String,
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
    ) -> Result<AdvMenuPath, NoValueFoundError> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("path_code", &menu_path_code);
        params.insert("application_code", &application_code);

        let menu_vec = MenuPath::find_by_params(sqlite_pool, params)
            .await
            .map_err(|err| {
                let msg = format!("Unable to fetch menu {:?}", err);
                NoValueFoundError::new(&msg)
            })?;
       
        if menu_vec.is_empty() {
            return Ok(Self {
                application_code,
                menu_path_code,
                menu_path: None,
                child_menu_paths: None,
                links: None,
                fields: None,
                key_fields: None,
                session_fields: None,
            });
        }
        let menu1 = menu_vec
            .first()
            .ok_or_else(|| NoValueFoundError::new("Unable to fetch menu path"))?
            .clone();
        let menu_path_id = menu1
            .id
            .as_ref()
            .ok_or_else(|| NoValueFoundError::new("Unable to fetch menu_path_id"))?
            .to_string();
        let menu: Option<MenuPath> = Some(menu1);
        let child_menus =
            AdvMenuPath::get_child_menus(sqlite_pool, &menu_path_code, &application_code).await;
        let (fields, key_fields) =
            AdvMenuPath::get_fields(sqlite_pool, menu_path_id.as_str()).await;
        Ok(Self {
            application_code,
            menu_path_code,
            menu_path: menu,
            child_menu_paths: child_menus,
            links: None,
            fields,
            key_fields,
            session_fields: None,
        })
    }

    async fn get_child_menus(
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        parent_path_code: &str,
        application_code: &str,
    ) -> Option<Vec<MenuPath>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("parent_path_code", parent_path_code);
        params.insert("application_code", application_code);
        let menus = MenuPath::find_by_params(sqlite_pool, params).await;
        match menus {
            Ok(menus) => Some(menus),
            Err(err) => {
                log::error!("Failed to fetch child menus, error : {:?}", err);
                None
            }
        }
    }



    pub async fn get_fields<'a>(
        sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        menu_path_id: &'a str,
    ) -> (Option<Vec<MenuFormField>>, Option<Vec<MenuFormField>>) {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("menu_path_id", menu_path_id);
        let fields: Option<Vec<MenuFormField>>;
        let key_fields: Option<Vec<MenuFormField>>;
        let fields1 = MenuFormField::find_by_params(sqlite_pool, params).await;
        if let Ok(fields2) = fields1 {
            let mut key_fields2: Vec<MenuFormField> = Vec::new();
            for f in &fields2 {
                if f.is_primary_key == Some(1) {
                    key_fields2.push(f.clone());
                }
            }
            fields = Some(fields2);
            key_fields = Some(key_fields2);
        } else {
            fields = None;
            key_fields = None;
        }

        (fields, key_fields)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_adv_menu() {
        let adv_menu =
            AdvMenuPath::new("Inoerp".to_string(), "PoHeaderEv".to_string(), Option::None)
                .await
                .expect("AdvMenuPath error");
        let _child_paths = adv_menu.child_menu_paths.expect("AdvMenuPath error");

    }
}
