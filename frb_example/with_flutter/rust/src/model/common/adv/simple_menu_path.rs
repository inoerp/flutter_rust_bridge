use crate::{
    app::system::error::no_value::NoValueFoundError,
    model::common::local::{menu_form_field::MenuFormField, menu_path::MenuPath},
};

use super::adv_menu_path::AdvMenuPath;
#[derive(Debug, Clone)]
pub struct SimpleMenuPath {
    pub menu_path: MenuPath,
    pub fields: Option<Vec<MenuFormField>>,
    pub key_fields: Option<Vec<MenuFormField>>,
}

impl SimpleMenuPath {
    pub async fn find_by_sql(
        pool: Option<&sqlx::Pool<sqlx::Sqlite>>,
        sql: &str,
    ) -> Result<Self, NoValueFoundError> {
        let child_menus = MenuPath::find_by_sql(pool, sql)
            .await
            .map_err(|_err| NoValueFoundError::new("No destination child menu found"))?;
        if child_menus.is_empty() {
            return Err(NoValueFoundError::new(
                format!("No destination child menu found for {sql}").as_str(),
            ));
        }
        let menu_path = child_menus
            .first()
            .ok_or_else(|| NoValueFoundError::new("no child_menus  found"))?;
        let menu_path_id = menu_path
            .id
            .ok_or_else(|| NoValueFoundError::new("no menu_path_id found"))?;
        let to_fields: (Option<Vec<MenuFormField>>, Option<Vec<MenuFormField>>) =
            AdvMenuPath::get_fields(pool, menu_path_id.to_string().as_str()).await;

        Ok(Self {
            menu_path: menu_path.clone(),
            fields: to_fields.0,
            key_fields: to_fields.1,
        })
    }
}
