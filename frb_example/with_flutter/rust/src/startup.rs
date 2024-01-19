use dotenv::dotenv;
use sqlx::{self, Pool};
use std::collections::HashMap;

use crate::app::system::init::app_init::AppInit;
use crate::app::system::log::itracing;
use crate::db::imysql::IMySql;
use crate::db::ipostgres::IPostgres;
use crate::db::isqlite::ISqlite;
use crate::model::common::local::rikdata_application::RikdataApplication;
use crate::model::state::global_state::GlobalState;
use crate::{
    app::cache::global_cache::GlobalCache,
    configuration::{self, Settings},
};

use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use super::{run, run_ssl};


pub async fn start() -> Result<(), std::io::Error> {
    init().await?;
    let settings = GlobalCache::get_settings()?;
    let global_state = GlobalCache::get_global_state()?;
    let address = format!("{}:{}", settings.host, settings.port);
    log::warn!("Starting server @{:?}", address);
    if settings.protocol.eq_ignore_ascii_case("https") {
        run_ssl(global_state, address).await
    } else {
        run(global_state, address).await
    }
}

pub async fn init() -> Result<bool, std::io::Error> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
   // env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn")); //warn or info
   dotenv().ok();
   // Panic if we can't read configuration
   let settings: configuration::Settings =
       configuration::get_configuration().expect("Failed to read configuration.");

    // Log all `tracing` events to files prefixed with `debug`. Since these
    // files will be written to very frequently, roll the log file every minute.
    let debug_file = rolling::minutely("./files/log/logs", "debug");
    // Log warnings and errors to a separate file. Since we expect these events
    // to occur less frequently, roll that file on a daily basis instead.
    let debug_level = AppInit::get_debug_level(&settings);
    let warn_file =
        rolling::daily("./files/log/logs", "warnings").with_max_level(debug_level);
    let all_files = debug_file.and(warn_file);
    println!("log files are available @ ./files/log/logs");

    tracing_subscriber::fmt()
        .with_writer(all_files)
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    itracing::write_all(6);



    let sqlite_pools: HashMap<String, Pool<sqlx::Sqlite>> = ISqlite::get_connection_pool()
        .await
        .expect("Unable to connect to Sqlite DB");

    let sqlite_pool = sqlite_pools.get("local");
    let all_applications: Vec<RikdataApplication> = RikdataApplication::find_all(sqlite_pool)
        .await
        .expect("Unable to find applications");

    let enabled_db_types = EnabledDatabaseType::new(settings.clone());

    let mysql_pools = set_mysql(&enabled_db_types, &settings, &all_applications).await;

    let postgres_pools = set_postgres(enabled_db_types, &settings, &all_applications).await;

    let settings_status = GlobalCache::set_settings(settings.clone())?;
    if settings_status {
        log::warn!("All settings are successfully cached")
    }
    let apps_status = GlobalCache::set_applications(all_applications.clone())?;
    if apps_status {
        log::warn!("All applications are successfully cached")
    }

    let js_objects = GlobalCache::set_js_objects()?;
    if js_objects {
        log::warn!("All java scripts are successfully cached")
    }

    let global_state: GlobalState = GlobalState::new(
        mysql_pools,
        postgres_pools,
        sqlite_pools,
        all_applications,
        settings.clone(),
    )
    .await;

    AppInit::init_db(&global_state).await?;
    GlobalCache::set_global_state(global_state.clone()).await?;
    Ok(true)
}

async fn set_postgres(
    enabled_db_types: EnabledDatabaseType,
    settings: &Settings,
    all_applications: &Vec<RikdataApplication>,
) -> HashMap<String, Pool<sqlx::Postgres>> {
    let postgres_pools: HashMap<String, Pool<sqlx::Postgres>>;
    if enabled_db_types.is_postgres {
        let pg_pools = IPostgres::get_connection_pools(settings, all_applications).await;
        match pg_pools {
            Ok(pool_map) => {
                postgres_pools = pool_map;
            }
            Err(_err) => {
                log::error!("No postgres db connection found");
                postgres_pools = HashMap::new();
            }
        }
    } else {
        postgres_pools = HashMap::new();
    }
    postgres_pools
}

async fn set_mysql(
    enabled_db_types: &EnabledDatabaseType,
    settings: &Settings,
    all_applications: &Vec<RikdataApplication>,
) -> HashMap<String, Pool<sqlx::MySql>> {
    let mysql_pools: HashMap<String, Pool<sqlx::MySql>>;
    if enabled_db_types.is_mysql {
        let pools = IMySql::get_connection_pools(settings, all_applications).await;
        match pools {
            Ok(pool_map) => {
                mysql_pools = pool_map;
            }
            Err(_err) => {
                log::error!("No mysql db connection found");
                mysql_pools = HashMap::new();
            }
        }
    } else {
        mysql_pools = HashMap::new();
    }
    mysql_pools
}

struct EnabledDatabaseType {
    is_postgres: bool,
    is_mysql: bool,
}

impl EnabledDatabaseType {
    pub fn new(settings: Settings) -> Self {
        let mut db_type = Self {
            is_mysql: false,
            is_postgres: false,
        };
        for setting in settings.db_settings {
            if setting.db_type.eq_ignore_ascii_case("postgres") {
                db_type.is_postgres = true;
            } else if setting.db_type.eq_ignore_ascii_case("mysql") {
                db_type.is_mysql = true;
            }
        }
        db_type
    }
}
