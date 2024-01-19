use crate::db::isqlite::DB_URL;
use sqlx::Pool;
use sqlx::{self, FromRow, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct MenuPath {
  pub id : Option<i32> ,
  #[serde(rename = "pathUrl")]
 pub path_url  : Option<String>,
  pub r#type : Option<String>,
  #[serde(rename = "applicationCode")]
 pub application_code  : Option<String>,
  #[serde(rename = "pathCode")]
 pub path_code  : Option<String>,
  #[serde(rename = "moduleName")]
 pub module_name  : Option<String>,
  #[serde(rename = "moduleApi")]
 pub module_api  : Option<String>,
  #[serde(rename = "parentPathUrl")]
 pub parent_path_url  : Option<String>,
  #[serde(rename = "parentPathCode")]
 pub parent_path_code  : Option<String>,
  #[serde(rename = "postPathCode")]
 pub post_path_code  : Option<String>,
  pub label : Option<String>,
  pub icon : Option<String>,
  pub description : Option<String>,
  #[serde(rename = "instanceName")]
 pub instance_name  : Option<String>,
  #[serde(rename = "postUrl")]
 pub post_url  : Option<String>,
  #[serde(rename = "defaultTrnx")]
 pub default_trnx  : Option<String>,
  #[serde(rename = "resultDisplay")]
 pub result_display  : Option<String>,
  #[serde(rename = "pathType")]
 pub path_type  : Option<String>,
  #[serde(rename = "entityDefinition")]
 pub entity_definition  : Option<String>,
  #[serde(rename = "navigationGroup")]
 pub navigation_group  : Option<String>,
  #[serde(rename = "allowGet")]
 pub allow_get  : Option<i32> ,
  #[serde(rename = "allowPost")]
 pub allow_post  : Option<i32> ,
  #[serde(rename = "allowPatch")]
 pub allow_patch  : Option<i32> ,
  #[serde(rename = "allowDelete")]
 pub allow_delete  : Option<i32> ,
  #[serde(rename = "usePathFromDataToPost")]
 pub use_path_from_data_to_post  : Option<i32> ,
  #[serde(rename = "enableSingleFormSubmission")]
 pub enable_single_form_submission  : Option<i32> ,
  pub sequence : Option<i32> ,
  #[serde(rename = "pathUrlAsChild")]
 pub path_url_as_child  : Option<String>,
  #[serde(rename = "alternateHost")]
 pub alternate_host  : Option<String>,
  #[serde(rename = "pathPreFix")]
 pub path_pre_fix  : Option<String>,
  #[serde(rename = "beforeGet")]
 pub before_get  : Option<String>,
  #[serde(rename = "afterGet")]
 pub after_get  : Option<String>,
  #[serde(rename = "beforePatch")]
 pub before_patch  : Option<String>,
  #[serde(rename = "afterPatch")]
 pub after_patch  : Option<String>,
  #[serde(rename = "beforePost")]
 pub before_post  : Option<String>,
  #[serde(rename = "afterPost")]
 pub after_post  : Option<String>,
  #[serde(rename = "beforeDelete")]
 pub before_delete  : Option<String>,
  #[serde(rename = "afterDelete")]
 pub after_delete  : Option<String>,
  #[serde(rename = "restTableName")]
 pub rest_table_name  : Option<String>,
  #[serde(rename = "postPathCodes")]
 pub post_path_codes  : Option<String>,
  #[serde(rename = "singleEntityForPost")]
 pub single_entity_for_post  : Option<i32> ,
  #[serde(rename = "asChildShowInMain")]
 pub as_child_show_in_main  : Option<i32> ,
  #[serde(rename = "printTemplateCode")]
 pub print_template_code  : Option<String>,
  #[serde(rename = "cascadePatch")]
 pub cascade_patch  : Option<i32> ,
  #[serde(rename = "cascadePost")]
 pub cascade_post  : Option<i32> ,
  #[serde(rename = "cascadeDelete")]
 pub cascade_delete  : Option<i32> ,
  #[serde(rename = "baseTableName")]
 pub base_table_name  : Option<String>,
  #[serde(rename = "formLayout")]
 pub form_layout  : Option<String>,
  #[serde(rename = "dataSourcePathCode")]
 pub data_source_path_code  : Option<String>,
  #[serde(rename = "dataSourceInstanceCode")]
 pub data_source_instance_code  : Option<String>,
  #[serde(rename = "dataSourcePathUrl")]
 pub data_source_path_url  : Option<String>,
  #[serde(rename = "fieldDisplayType")]
 pub field_display_type  : Option<String>,
  #[serde(rename = "contentViewTemplateId")]
 pub content_view_template_id  : Option<i32> ,
  #[serde(rename = "oldId")]
 pub old_id  : Option<i32> ,
  #[serde(rename = "boardViewXaxis")]
 pub board_view_xaxis  : Option<String>,
  #[serde(rename = "boardViewYaxis")]
 pub board_view_yaxis  : Option<String>,
}

impl MenuPath {

        pub async fn find_by_sql(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            sql: &str,
        ) -> Result<Vec<MenuPath>, Box<dyn Error>> {
            if let Some(pool) = sqlite_pool {
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<MenuPath> = sqlx::query_as::<_, MenuPath>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            } else {
                let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await?;
                let mut connection = pool.acquire().await?;
                let data_rows: Vec<MenuPath> = sqlx::query_as::<_, MenuPath>(sql)
                    .fetch_all(&mut connection)
                    .await?;
                Ok(data_rows)
            }
        }

        pub async fn find_all(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        ) -> Result<Vec<MenuPath>, Box<dyn Error>> {
            let sql: &str = "SELECT * from menu_path ";
           Self::find_by_sql(sqlite_pool, sql).await
        }

        pub async fn find_by_id(
            sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
            id: &str,
        ) -> Result<Vec<MenuPath>, Box<dyn Error>> {
            let sql1: String = format!("SELECT * from menu_path WHERE id = '{}' ", id);
            Self::find_by_sql(sqlite_pool, sql1.as_str()).await
        }

    pub async fn find_by_params(
    sqlite_pool: Option<&Pool<sqlx::Sqlite>>,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<MenuPath>, Box<dyn Error>> {
        let mut sql: String = "SELECT * from menu_path WHERE  1 = 1 ".to_string();
        let mut param_values: Vec<&str> = Vec::new();
         if !params.is_empty() {
            for (k, v) in params {
                sql = sql + " AND " + k + " =  ? ";
                param_values.push(v);
            }
        }

        let mut x1 = sqlx::query_as::<_, MenuPath>(&sql);
        for val in param_values {
            x1 = x1.bind(val);
        }

        if let Some(pool) = sqlite_pool {
        let mut connection = pool.acquire().await?;
            let data_rows: Vec<MenuPath> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        } else {
            let pool = SqlitePool::connect(DB_URL).await?;
            let mut connection = pool.acquire().await?;
            let data_rows: Vec<MenuPath> = x1.fetch_all(&mut connection).await?;
            Ok(data_rows)
        }
    }
}

#[cfg(test)]
mod tests{
use super::*;

#[tokio::test]
async fn test_find_all() {
    let data = MenuPath::find_all(Option::None).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d.len());
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let data = MenuPath::find_by_id(Option::None,"1").await;
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
    let data = MenuPath::find_by_params(Option::None, params).await;
    match data {
        Ok(d) => {
            log::info!("found data. length {:?}", d);
        }
        Err(err) => log::info!("failed to find data , error {:?} ", err),
    }
}

}


