use std::collections::HashMap;

use crate::app::js::db::action::JsDbAction;
use crate::model::type_def::common::CommunicationChannel;
use std::sync::mpsc;

use crate::db::sql_data::SqlActionType;
use js_sandbox::api::{serde_v8, FunctionCallbackArguments, HandleScope, ReturnValue};
use js_sandbox::exposed_func::ExposedFunction;

use tokio::runtime::Runtime;

pub struct SqlUpdate {}
impl ExposedFunction for SqlUpdate {
    fn rust_func_for_js(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) {
        complete_execution(scope, args, rv, SqlActionType::Update)
    }

    fn name() -> String {
        "sqlUpdate".to_string()
    }
}

pub struct SqlInsert {}
impl ExposedFunction for SqlInsert {
    fn rust_func_for_js(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) {
        complete_execution(scope, args, rv, SqlActionType::Insert)
    }

    fn name() -> String {
        "sqlInsert".to_string()
    }
}

pub struct SqlDelete {}
impl ExposedFunction for SqlDelete {
    fn rust_func_for_js(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) {
        complete_execution(scope, args, rv, SqlActionType::Delete)
    }

    fn name() -> String {
        "SqlDelete".to_string()
    }
}


fn complete_execution(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
    action: SqlActionType,
) {
    let arg: HashMap<String, String> = match serde_v8::from_v8(scope, args.get(0)) {
        Ok(data) => data,
        Err(err) => {
            log::error!("Unable to get arg1. Error {:?} ", err);
            return;
        }
    };

    let (tx, rx): CommunicationChannel = mpsc::channel();

    std::thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(err) => {
                log::error!("Failed to create Runtime: {}", err);
                return;
            }
        };

        let action = JsDbAction::new(action, arg);
        let data = match rt.block_on(action.complete_task()) {
            Ok(data) => data,
            Err(err) => {
                log::error!("Failed to complete JS task: {}", err);
                return;
            }
        };

        if let Err(err) = tx.send(Ok(data)) {
            log::error!("Failed to send data: {}", err);
        }
    });

    let ret_data = match rx.recv() {
        Ok(Ok(data)) => data,
        Ok(Err(err)) => {
            log::error!("Error: {}", err);
            return;
        }
        Err(err) => {
            log::error!("Channel receive error: {}", err);
            return;
        }
    };

    match serde_v8::to_v8(scope, &ret_data) {
        Ok(result_value) => rv.set(result_value),
        Err(err) => {
            //throw_type_error(scope, "Failed to convert result to V8 value");
            log::error!("serde_v8 error : {:?}", err);
        }
    }
}
