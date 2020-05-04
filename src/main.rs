use std::fs;
use std::env;
use std::process;
use std::rc::Rc;

mod expressions;
mod errors;
mod classes;

use expressions::Expression;

pub struct Config {
  filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("missing filename!")
    }

    let filename = args[1].clone();

    Ok(Config {
      filename,
    })
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
    Ok(val) => println!("Oak: Result: {:?}", &*val),
    Err(e) => eprintln!("Oak: Interpreter error: {}", e.reason)
  };
}

pub fn run(prgm: String, _config: Config) -> Result<Rc<classes::Value>, errors::EvalError> {
  let mut str_iter = classes::StringIterator::new(&prgm);
  let main_expression = expressions::BlockExpression::new(&mut str_iter)?;
  let prgm_scope = classes::Scope::new(None);

  main_expression.evaluate(&prgm_scope, Rc::new(classes::Value::Null))
}
