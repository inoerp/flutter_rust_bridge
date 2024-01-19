use crate::{
    app::{system::error::no_value::NoValueFoundError, utils::url::AppUrl},
    db::{action::ActionData, db_execution::DbExecution, sql_data::SqlActionType},
    model::state::global_state::GlobalState, configuration::Settings,
};

pub struct AppInit;

impl AppInit {
    pub async fn init_db(gs: &GlobalState) -> Result<(), NoValueFoundError> {
        if !gs.settings.app_settings.run_init_scripts {
            return Ok(());
        }

        let base_url = AppUrl::get_base_url()?;
        let host_name = base_url.clone() + "/api";
        let login_url = base_url + "/auth/login";

        let update_sql = format!("UPDATE app_instance SET host_name = '{}', login_url = '{}', host_port = '{}'
         where authorization_type = 'JWT'
         AND type = 'oracleCloudSimilar'
         AND instance_name NOT LIKE '%Demo%' ",
                         host_name, login_url, gs.settings.port);
        let pools = gs.conn_pools.lock().await;
        let db_mapping = pools.get("local").ok_or_else(|| {
            NoValueFoundError::new("Unable to get db connection mapping for init_db action")
        })?;

        let ad =
            ActionData::int_for_sql( SqlActionType::Update, update_sql, vec![]);
        DbExecution::execute(db_mapping, &ad).await?;

        Ok(())
    }

    pub fn get_debug_level(settings: &Settings) -> tracing::Level{
        match settings.app_settings.debug_level {
            1 => tracing::Level::ERROR,
            2 => tracing::Level::WARN,
            3 => tracing::Level::INFO,
            4 => tracing::Level::DEBUG,
            5 => tracing::Level::TRACE,
            _=>  tracing::Level::ERROR
        }

    }
}
