/* Defines platform agnostic functionality for compilation
*/

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod list;
mod errors;

pub struct Config {
    pub program: String,
}

impl Config {
    pub fn new(program: String) -> Self {
        Config { program }
    }

    pub fn compile(&self) -> Result<list::List, errors::CompileError> {
        let ast = list::List::from_text(&self.program);
        // Compile the program
        
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_oak(program: String) -> JsValue {
    match Config::new(program).run() {
        Ok(val) => JsValue::from_str(&val.to_string()),
        Err(e) => JsValue::from_str(&e.to_string()),
    }
}

// expect logging function (log_oak) to be exposed globally in the JS
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn log_oak(s: &str);
}
