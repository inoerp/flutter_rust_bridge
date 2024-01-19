use std::thread;

use js_sandbox::exposed_func::ExposedFunction;
use tokio::runtime::Runtime;

use crate::startup;

pub struct JsDbConnection {}

impl JsDbConnection{
    async fn init() -> Result<bool, std::io::Error>{
       let data = startup::init().await?;
       Ok(data)
    }
}

impl ExposedFunction for JsDbConnection {
    fn rust_func_for_js(
        _scope: &mut js_sandbox::api::HandleScope,
        _args: js_sandbox::api::FunctionCallbackArguments,
        _rv: js_sandbox::api::ReturnValue,
    ) {
        thread::spawn(move || {
            let rt = Runtime::new().expect("Unable to create js RunTime");
            let data = rt.block_on(Self::init());
            match data {
                Ok(res) => log::info!("successfully initialized db connection for js. value {:?}", res),
                Err(err) => log::error!("Error in initializing db connection for js .  {:?}", err),
            }
        })
        .join()
        .expect("Unable to add js RunTime");
    }

    fn name() -> String {
        "init_db_connection".to_string()
    }
}
