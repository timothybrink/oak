use std::env;
use std::fs;
use std::process;
use std::rc::Rc;

mod classes;
mod errors;
mod expressions;
mod stdlib;

use expressions::Expression;

pub struct Config {
    filename: String,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Oak: {}", err);
        process::exit(1);
    });

    let prgm: String = fs::read_to_string(&config.filename).unwrap_or_else(|_err| {
        eprintln!("Oak: file not found!");
        process::exit(1);
    });

    match run(prgm, config) {
        Ok(val) => println!("Oak - result: {:#?}", &*val),
        Err(e) => eprintln!("Oak - interpreter error: {}", e.reason),
    };
}

pub fn run(prgm: String, _config: Config) -> Result<Rc<classes::Value>, errors::EvalError> {
    let mut str_iter = classes::StringIterator::new(&prgm);

    // create a block expression that contains all the expressions in the prelude,
    // plus another block expression containing the file contents
    let mut expressions = stdlib::get_prelude();
    expressions.push(Rc::new(expressions::BlockExpression::new(&mut str_iter)?));
    let main_expression = crate::expressions::BlockExpression { expressions };
    let mut prgm_scope = classes::Scope::new(None);

    // insert stdlib
    stdlib::insert_stdlib(&mut prgm_scope);

    main_expression.evaluate(Rc::new(prgm_scope), Rc::new(classes::Value::Null))
}
