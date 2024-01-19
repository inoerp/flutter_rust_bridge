// use rhai::{Engine, EvalAltResult};

// use std::fs::File;
// use std::io::Read;

// fn read_file() -> String {
//     // Specify the file path
//     let file_path = "./assets/rhai/rhai1.rhai";

//     // Open the file
//     let mut file = match File::open(file_path) {
//         Ok(file) => file,
//         Err(e) => panic!("Failed to open file: {}", e),
//     };

//     // Read the file contents into a string
//     let mut contents = String::new();
//     if let Err(e) = file.read_to_string(&mut contents) {
//         panic!("Failed to read file: {}", e);
//     }

//     // Print the file contents
//     contents
// }

#[cfg(test)]
mod tests {
    use rhai::{Engine, EvalAltResult};

    // use std::fs::File;
    // use std::io::Read;

    #[test]
    pub fn test_main() -> Result<(), Box<EvalAltResult>> {
        // Create an 'Engine'
        let engine = Engine::new();

        // Your first Rhai Script
        let script = "print(40 + 2);";

        // Run the script - prints "42"
        engine.run(script)?;

        let script2 = "print(\"hello world\");";
        engine.run(script2)?;

        let _result = engine.eval::<i32>("40 + 2")?;

        // Done!
        Ok(())
    }

    #[test]
    pub fn file_test() -> Result<(), Box<EvalAltResult>> {
        // Create an 'Engine'
        let engine = Engine::new();

        //engine.eval_file::<i32>("rhai1.rhai".into())?;

        // engine.run_file("./assets/rhai/rhai1.rhai".into())?;

        let _result = engine.eval_file::<i64>("./assets/rhai/rhai1.rhai".into())?;

        // Done!
        Ok(())
    }
}
