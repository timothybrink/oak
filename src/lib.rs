use std::rc::Rc;

mod common;
mod expressions;
mod stdlib;

use expressions::Expression;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("missing filename!");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(prgm: String, _config: Config) -> Result<Rc<common::Value>, common::EvalError> {
    let mut str_iter = common::StringIterator::new(&prgm);

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
