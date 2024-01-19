use std::collections::HashMap;

use crate::app::js::db::action::JsDbAction;
use crate::model::type_def::common::CommunicationChannel;
use std::sync::mpsc;

use crate::db::sql_data::SqlActionType;
use js_sandbox::api::{serde_v8, FunctionCallbackArguments, HandleScope, ReturnValue};
use js_sandbox::exposed_func::ExposedFunction;
use tokio::runtime::Runtime;

pub struct SqlSelect {}
impl ExposedFunction for SqlSelect {
    fn rust_func_for_js(
        scope: &mut HandleScope,
        args: FunctionCallbackArguments,
        mut rv: ReturnValue,
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

            let action = JsDbAction::new(SqlActionType::Select, arg);
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
            },
            Err(err) => {
                log::error!("Channel receive error: {}", err);
                return;
            },
        };

        match serde_v8::to_v8(scope, &ret_data) {
            Ok(result_value) => rv.set(result_value),
            Err(err) => {
                log::error!("serde_v8 error : {:?}", err);
            }
        }
    }

    fn name() -> String {
        "sqlSelect".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::app::js::{
        db::{
            connection::JsDbConnection,
            console::ConsoleLog,
            sql_execute::{SqlDelete, SqlInsert, SqlUpdate},
        },
        entity::{js_input::JsRustInput, js_output::JsOutput},
    };
    use js_sandbox::Script;

    use super::*;

    #[test]
    fn file_read_test() {
        let mut isolate = Script::rd_get_run_time().expect("Unable to find js run time");
        isolate.add_exposed_func::<JsDbConnection>();
        // isolate
        //     .rd_run_string("db_connection(); ")
        //     .expect("Error in getting data");
        isolate.add_exposed_func::<SqlSelect>();
        isolate.add_exposed_func::<SqlInsert>();
        isolate.add_exposed_func::<SqlUpdate>();
        isolate.add_exposed_func::<SqlDelete>();
        isolate.add_exposed_func::<ConsoleLog>();

        let file_path = "./assets/js/ierp/global.js";
        isolate
            .rd_run_file(file_path)
            .expect("Unable to find global file");

        let file_path = "./assets/js/ierp/inv/inv_transaction_doc_line.js";
        isolate
            .rd_run_file(file_path)
            .expect("Unable to find file");

        let mut data2: HashMap<String, String> = HashMap::new();
        data2.insert("invTransactionDocHeaderId".to_string(), "1".to_string());
        let data_str: serde_json::Value = serde_json::json!(&data2);
        let data = JsRustInput::new(data_str.clone(), data_str, "".to_string(), "".to_string());

        let tup = (data,);

        let _ret_data: JsOutput = isolate
            .call("beforeGet", tup)
            .expect("Error in fetching data");

    }

    #[test]
    fn unit_test() {
        let js_code = "console.log('hello from js'); 
    init_db_connection();
    let request = {
        sql: 'SELECT * from am_asset',
        dbType: 'MySQL',
        connName: 'Inoerp'
      };
    const result = sqlSelect(request);

    // Handle the result
	console.log('value returned from rust');
    console.log(result);

	const resultObj = JSON.parse(result); // Parse the JSON string into an object
    console.log(resultObj[0].description);
    console.log(resultObj[1].description);

    function get_val(){
        return {
            rd_proceed_status: true,
            rd_error_message: '',
            rd_proceed_message: 'Successfully completed'
        }
    }";
        let mut isolate = Script::rd_get_run_time().expect("Unable to find js run time");
        isolate.add_exposed_func::<JsDbConnection>();
        // isolate
        //     .rd_run_string("db_connection(); ")
        //     .expect("Error in getting data");
        isolate.add_exposed_func::<SqlSelect>();
        isolate.add_exposed_func::<SqlInsert>();
        isolate.add_exposed_func::<SqlUpdate>();
        isolate.add_exposed_func::<SqlDelete>();

        isolate
            .rd_run_string(js_code)
            .expect("Error in getting data");
        let _ret_data: JsOutput = isolate.call("get_val", ()).expect("Error in fetching data");
        let _ret_data: JsOutput = isolate.call("get_val", ()).expect("Error in fetching data");
    }
}
