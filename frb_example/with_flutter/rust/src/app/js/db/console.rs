use js_sandbox::api::{serde_v8, FunctionCallbackArguments, HandleScope, ReturnValue};
use js_sandbox::exposed_func::ExposedFunction;


pub struct ConsoleLog {}
impl ExposedFunction for ConsoleLog {
    fn rust_func_for_js(
        scope: &mut HandleScope,
        args: FunctionCallbackArguments,
        _rv: ReturnValue,
    ) {
        let _arg: String = match serde_v8::from_v8(scope, args.get(0)) {
            Ok(data) => data,
            Err(err) => {
                log::error!("Unable to get arg1. Error {:?} ", err);
                return;
            }
        };
    }

    fn name() -> String {
        "consoleLog".to_string()
    }
}

