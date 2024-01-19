use std::{collections::HashMap, str::FromStr, sync::Arc};

use js_sandbox::{self, Script};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::sync::Mutex;

use crate::app::{
    js::db::{
        connection::JsDbConnection,
        console::ConsoleLog,
        sql_execute::{SqlDelete, SqlInsert, SqlUpdate},
        sql_select::SqlSelect,
    },
    system::error::no_value::NoValueFoundError,
};

use super::{js_input::JsRustInput, js_output::JsOutput, js_trigger_point::JsTriggerPoint};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsPrimaryObject {
    pub base_path: String,
    pub entity_path: String,
    pub trigger_point: JsTriggerPoint,
    pub func_name: String,
    pub files: Vec<String>,
}

pub enum JsRunTime{
    Runtime(Arc<Mutex<Script>>),
    None
}

//TODO remove hardcoded value ierp
impl JsPrimaryObject {
    pub fn run(&self, input_data: JsRustInput) -> Result<JsOutput, NoValueFoundError> {
        let mut isolate: Script = self.get_isolate()?;

        for f in &self.files {
            let file_path = "./assets/js/".to_string() + self.base_path.as_str() + "/" + f.as_str();
            isolate.rd_run_file(&file_path).map_err(|err| {
                NoValueFoundError::new(
                    format!(
                        "Unable to find file.\nFile path:{:?} \nError {:?}",
                        file_path, err
                    )
                    .as_str(),
                )
            })?;
        }

        let tup = (input_data,);

        let res: JsOutput = isolate.call(&self.func_name, tup).map_err(|err| {
            log::error!("Error in running js {:?} ", err);
            NoValueFoundError::new(
                format!(
                    "Unable to call function :{:?} \nError {:?}",
                    &self.func_name, err
                )
                .as_str(),
            )
        })?;

        //convert the result to a hashmap of JsPrimaryObject
        // let result: Map<String, Value> = serde_json::from_str(&res)
        // .map_err(|err| {
        //     NoValueFoundError::new(format!("Unable to deserialize result into HashMap<String, JsPrimaryObject>. Error {:?}", err).as_str())
        // })?;
        Ok(res)
    }

    pub fn get_isolate(&self) -> Result<Script, NoValueFoundError> {
        let mut isolate: Script = Script::rd_get_run_time().map_err(|err| {
            NoValueFoundError::new(format!("Unable to get run time. Error {:?}", err).as_str())
        })?;
        if self.files.is_empty() {
            return Err(NoValueFoundError::new("No js script files found"));
        }
        isolate.add_exposed_func::<ConsoleLog>();
        isolate.add_exposed_func::<JsDbConnection>();
        isolate.add_exposed_func::<SqlSelect>();
        isolate.add_exposed_func::<SqlInsert>();
        isolate.add_exposed_func::<SqlUpdate>();
        isolate.add_exposed_func::<SqlDelete>();
        let file_path = "./assets/js/".to_string() + self.base_path.as_str() + "/global.js";
        isolate.rd_run_file(&file_path).map_err(|err| {
            NoValueFoundError::new(
                format!(
                    "Unable to find global file.\nFile path:{:?} \nError {:?}",
                    file_path, err
                )
                .as_str(),
            )
        })?;
        Ok(isolate)
    }

    pub fn init_from_asset() -> Result<HashMap<String, JsPrimaryObject>, NoValueFoundError> {
        let mut script: Script = Script::rd_get_run_time().map_err(|err| {
            NoValueFoundError::new(format!("Unable to get run time. Error {:?}", err).as_str())
        })?;
        script
            .rd_run_file("./assets/js/ierp/main.js")
            .map_err(|err| {
                NoValueFoundError::new(format!("Unable to find file. Error {:?}", err).as_str())
            })?;

        let res: String = script.call("main", ()).map_err(|err| {
            NoValueFoundError::new(
                format!("Unable to call main function. Error {:?}", err).as_str(),
            )
        })?;

        //convert the result to a hashmap of JsPrimaryObject
        let result: Map<String, Value> = serde_json::from_str(&res)
        .map_err(|err| {
            NoValueFoundError::new(format!("Unable to deserialize result into HashMap<String, JsPrimaryObject>. Error {:?}", err).as_str())
        })?;

        let mut map_of_primary_objects: HashMap<String, JsPrimaryObject> = HashMap::new();

        for (key, val) in result {
            let js_value = val
                .as_object()
                .ok_or_else(|| NoValueFoundError::new("Unable to get object from js_value"))?;
            for (k1, v1) in js_value {
                let arr = v1
                    .as_array()
                    .ok_or_else(|| NoValueFoundError::new("Unable to convert to array"))?;
                let files: Vec<String> = arr
                    .get(0)
                    .ok_or_else(|| NoValueFoundError::new("js values is missing"))?
                    .as_str()
                    .ok_or_else(|| NoValueFoundError::new("js values is missing 2"))?
                    .to_string()
                    .split(',')
                    .map(String::from)
                    .collect();
                let func_name = arr
                    .get(1)
                    .ok_or_else(|| NoValueFoundError::new("Unable to get index 1"))?
                    .as_str()
                    .ok_or_else(|| NoValueFoundError::new("Unable to get index 1 as str"))?
                    .to_string();
                let tp = JsTriggerPoint::from_str(k1).map_err(|e| {
                    NoValueFoundError::new(format!("Invalid trigger point {:?}", e).as_str())
                })?;
                let obj = JsPrimaryObject {
                    base_path: "ierp".to_string(),
                    entity_path: key.clone(),
                    trigger_point: tp,
                    func_name,
                    files,
                };
                let new_key = "ierp".to_string() + "__" + key.clone().as_str() + "__" + k1.as_str();
                map_of_primary_objects.insert(new_key, obj);
            }
        }
        Ok(map_of_primary_objects)
    }
}
