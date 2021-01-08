use std::rc::Rc;

mod common;
mod expressions;
mod stdlib;
mod util;

use expressions::Expression;

pub struct Config {
    pub program: String,
}

impl Config {
    pub fn new(program: String) -> Self {
        Config { program }
    }

    pub fn run(&self) -> Result<Rc<common::Value>, common::EvalError> {
        let mut str_iter = common::StringIterator::new(&self.program);

        // create a block expression that contains all the expressions in the prelude,
        // plus another block expression containing the file contents
        let mut expressions = stdlib::get_prelude();
        expressions.push(Rc::new(expressions::BlockExpression::new(&mut str_iter)?));
        let main_expression = crate::expressions::BlockExpression { expressions };
        let mut prgm_scope = common::Scope::new(None);

        // insert stdlib
        stdlib::insert_stdlib(&mut prgm_scope);

        main_expression.evaluate(Rc::new(prgm_scope), Rc::new(common::Value::Null))
    }
}
