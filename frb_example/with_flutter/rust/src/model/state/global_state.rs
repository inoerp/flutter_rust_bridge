use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::configuration::Settings;

use crate::db::model::{
    db_conn_map::{ DbConnMapping, DbPool},
    db_type::DbType,
};
use crate::model::common::local::{
    app_instance::AppInstance, rikdata_application::RikdataApplication,
};

use sqlx::Pool;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct GlobalState {
    //pub conn_pools: Arc<Mutex<HashMap<String, DbConnMapping>>>,//current key base_path 
    // ARC not required as internally, actix web::Data uses Arc
    pub conn_pools: Arc<Mutex<HashMap<String, DbConnMapping>>>,//current key base_path
     //key needs to be changed to db_settings name from the base path
    //TODO complete key change
    pub app_instances: Option<Vec<AppInstance>>,
    pub applications: HashMap<String, RikdataApplication>, //Key is the base path
    // pub mysql_pools: HashMap<String, Pool<sqlx::MySql>>, //key is the name of the base path
    // pub postgres_pools: HashMap<String, Pool<sqlx::Postgres>>, //key is the name of the base path
    pub sqlite_pools: HashMap<String, Pool<sqlx::Sqlite>>,
    pub settings: Settings,
    //pub js_objects: HashMap<String, js_object::JsPrimaryObject> //TODO remove and use global_cache
}

impl  GlobalState {
    pub async fn new(
        mysql_pools: HashMap<String, Pool<sqlx::MySql>>,
        postgres_pools: HashMap<String, Pool<sqlx::Postgres>>,
        sqlite_pools: HashMap<String, Pool<sqlx::Sqlite>>,
        all_applications: Vec<RikdataApplication>,
        settings: Settings,
        //js_objects: HashMap<String, js_object::JsPrimaryObject>
    ) -> GlobalState {
        let mut conn_pools = Arc::new(Mutex::new(HashMap::new()));
        let sqlite_pool = sqlite_pools.get("local");
        let app_instances = match AppInstance::find_all(sqlite_pool).await {
            Ok(instances) => Some(instances),
            Err(msg) => {
                log::error!("Unable to initialize app_instances: {}", msg);
                None
            }
        };
        let mut applications: HashMap<String, RikdataApplication> = HashMap::new();
        create_mappings(
            all_applications,
            &mysql_pools,
            &mut conn_pools,
            &postgres_pools,
            &sqlite_pools,
            &mut applications,
           // &settings,
        ).await;
        GlobalState  {
            conn_pools,
            sqlite_pools,
            app_instances,
            applications,
            settings,
           // js_objects,
        }
    }

    pub fn get_application_map(
        all_applications: &Vec<RikdataApplication>,
    ) -> HashMap<String, RikdataApplication> {
        let mut applications: HashMap<String, RikdataApplication> = HashMap::new();
        for app in all_applications {
            if let Some(base_path) = &app.base_path {
                applications.insert(base_path.to_string(), app.clone());
            }
        }
        applications
    }

    pub fn get_app_for_base_path(
        &self,
        base_path: &str,
    ) -> Result<&RikdataApplication, Box<dyn Error>> {
        match self.applications.get(base_path) {
            Some(val) => Ok(val),
            None => Err(Box::new(BasePathError {
                message: "No application found for the base path".to_string(),
                path: base_path.to_string(),
            })),
        }
    }


}
async fn create_mappings(
    all_applications: Vec<RikdataApplication>,
    mysql_pools: &HashMap<String, Pool<sqlx::MySql>>,
    conn_pools: &mut  Arc<Mutex<HashMap<String, DbConnMapping>>>,
    postgres_pools: &HashMap<String, Pool<sqlx::Postgres>>,
    sqlite_pools: &HashMap<String, Pool<sqlx::Sqlite>>,
    applications: &mut HashMap<String, RikdataApplication>,
) {
    for app in all_applications {
        if let Some(base_path) = &app.base_path {
            if let Some(db_type_str) = &app.db_type {
                let db_type = DbType::from_string(db_type_str);
                match db_type {
                    DbType::MySql => {
                        if let Some(pool) = mysql_pools.get(base_path) {
                            let db_conn_map = DbConnMapping {
                                base_path: base_path.clone(),
                                db_type: db_type.clone(),
                                conn_pool: DbPool::MySql(pool.clone()),
                            };
                            conn_pools.lock().await.insert(base_path.to_string(), db_conn_map);
                        }
                    }
                    DbType::Postgres => {
                        if let Some(pool) = postgres_pools.get(base_path) {
                            let db_conn_map = DbConnMapping {
                                base_path: base_path.clone(),
                                db_type: db_type.clone(),
                                conn_pool: DbPool::Postgres(pool.clone()),
                            };
                            conn_pools.lock().await.insert(base_path.to_string(), db_conn_map);
                        }
                    }
                    DbType::Sqlite => {
                        if let Some(pool) = sqlite_pools.get(base_path) {
                            let db_conn_map = DbConnMapping {
                                base_path: base_path.clone(),
                                db_type: db_type.clone(),
                                conn_pool: DbPool::Sqlite(pool.clone()),
                            };
                            conn_pools.lock().await.insert(base_path.to_string(), db_conn_map);
                        }
                    }
                    DbType::MsSql => {},
                    DbType::Oracle => {},
                }
            }
            applications.insert(base_path.to_string(), app);
        }
    }
}

#[derive(Debug)]
struct BasePathError {
    message: String,
    path: String,
}

impl std::fmt::Display for BasePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} : {}", self.message, self.path)
    }
}

impl std::error::Error for BasePathError {}
