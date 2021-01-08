use std::rc::Rc;

pub mod common;
mod expressions;
mod stdlib;

use common::Value;
use expressions::Expression;

/// This trait is used to provide interfaces to slightly more platform-dependant calls like log and exit.
/// For example, the handling on WASM would be different than on native.
pub trait NativeInterface {
    fn log(&self, msg: Rc<Value>) -> ();
    fn exit(&self, code: i32) -> !;
}

pub struct Config {
    pub program: String,
    pub sys_int: Rc<dyn NativeInterface>,
}

impl Config {
    pub fn new(program: String, sys_int: Rc<dyn NativeInterface>) -> Self {
        Config { program, sys_int }
    }

    pub fn run(&self) -> Result<Rc<common::Value>, common::EvalError> {
        let mut str_iter = common::StringIterator::new(&self.program);

        // create a block expression that contains all the expressions in the prelude,
        // plus another block expression containing the file contents
        let mut expressions = stdlib::get_prelude();
        expressions.push(Rc::new(expressions::BlockExpression::new(&mut str_iter)?));
        let main_expression = crate::expressions::BlockExpression { expressions };
        let mut prgm_scope = common::Scope::new_global(Rc::clone(&self.sys_int));

        // insert stdlib
        stdlib::insert_stdlib(&mut prgm_scope);

        main_expression.evaluate(Rc::new(prgm_scope), Rc::new(common::Value::Null))
    }
}
