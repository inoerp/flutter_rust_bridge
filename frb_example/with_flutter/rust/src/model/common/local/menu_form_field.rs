use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct MenuFormField {
  pub id : Option<i32> ,
  #[serde(rename = "menuPathId")]
 pub menu_path_id  : Option<i32> ,
  pub name : Option<String>,
  #[serde(rename = "parentFieldName")]
 pub parent_field_name  : Option<String>,
  pub label : Option<String>,
  pub title : Option<String>,
  #[serde(rename = "fieldGroup")]
 pub field_group  : Option<String>,
  pub description : Option<String>,
  #[serde(rename = "inputType")]
 pub input_type  : Option<String>,
  #[serde(rename = "defaultValue")]
 pub default_value  : Option<String>,
  #[serde(rename = "maxLength")]
 pub max_length  : Option<i32> ,
  #[serde(rename = "isReadonly")]
 pub is_readonly  : Option<i32> ,
  #[serde(rename = "isHidden")]
 pub is_hidden  : Option<i32> ,
  #[serde(rename = "isPrimaryKey")]
 pub is_primary_key  : Option<i32> ,
  #[serde(rename = "isIntransient")]
 pub is_intransient  : Option<i32> ,
  #[serde(rename = "isForeignKey")]
 pub is_foreign_key  : Option<i32> ,
  #[serde(rename = "fieldFormula")]
 pub field_formula  : Option<String>,
  #[serde(rename = "isReadonlyAfterInsert")]
 pub is_readonly_after_insert  : Option<i32> ,
  #[serde(rename = "isPrintable")]
 pub is_printable  : Option<i32> ,
  #[serde(rename = "isDisabled")]
 pub is_disabled  : Option<i32> ,
  #[serde(rename = "validationFormula")]
 pub validation_formula  : Option<String>,
  #[serde(rename = "javaScriptFunction")]
 pub java_script_function  : Option<String>,
  #[serde(rename = "isNotSelectable")]
 pub is_not_selectable  : Option<i32> ,
  #[serde(rename = "isUniqueKey")]
 pub is_unique_key  : Option<i32> ,
  #[serde(rename = "isRequired")]
 pub is_required  : Option<i32> ,
  #[serde(rename = "isNotPosted")]
 pub is_not_posted  : Option<i32> ,
  #[serde(rename = "isPostedOnPatch")]
 pub is_posted_on_patch  : Option<i32> ,
  #[serde(rename = "isNotCopied")]
 pub is_not_copied  : Option<i32> ,
  #[serde(rename = "isConfirmationRequired")]
 pub is_confirmation_required  : Option<i32> ,
  #[serde(rename = "isRequiredInSearch")]
 pub is_required_in_search  : Option<i32> ,
  #[serde(rename = "buttonPath")]
 pub button_path  : Option<String>,
  #[serde(rename = "buttonFunction")]
 pub button_function  : Option<String>,
  #[serde(rename = "buttonFunctionName")]
 pub button_function_name  : Option<String>,
  #[serde(rename = "sourceFieldName")]
 pub source_field_name  : Option<String>,
  #[serde(rename = "selectPathCode")]
 pub select_path_code  : Option<String>,
  #[serde(rename = "selectPathCodeFieldName")]
 pub select_path_code_field_name  : Option<String>,
  #[serde(rename = "lovHrefPath")]
 pub lov_href_path  : Option<String>,
  #[serde(rename = "selectPathCodeFieldLabel")]
 pub select_path_code_field_label  : Option<String>,
  pub sequence : Option<String>,
  #[serde(rename = "selectList")]
 pub select_list  : Option<String>,
  #[serde(rename = "selectOtherFieldMappings")]
 pub select_other_field_mappings  : Option<String>,
  #[serde(rename = "controlField")]
 pub control_field  : Option<String>,
  #[serde(rename = "applicationCode")]
 pub application_code  : Option<String>,
  #[serde(rename = "dbColumnName")]
 pub db_column_name  : Option<String>,
  #[serde(rename = "oldId")]
 pub old_id  : Option<i32> ,
}

impl MenuFormField {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<MenuFormField>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<MenuFormField> = sqlx::query_as::<_, MenuFormField>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<MenuFormField> = sqlx::query_as::<_, MenuFormField>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<MenuFormField>, Box<dyn Error>> {
            let sql: &str = "SELECT * from menu_form_field ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<MenuFormField>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from menu_form_field WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<MenuFormField>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from menu_form_field WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, MenuFormField>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<MenuFormField> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<MenuFormField> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = MenuFormField::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = MenuFormField::find_by_id(Option::None,"1").await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_params() {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("type", "oracleCloudSimilar");
    let data = MenuFormField::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


