use std::env;
use std::fs;
use std::process;
use std::rc::Rc;

use oak::Config;
use oak::NativeInterface;
use oak::common::Value;

struct StdInterface {}

impl NativeInterface for StdInterface {
    fn log(&self, msg: Rc<Value>) -> () {
        println!("{:?}", msg);
    }

    fn exit(&self, code: i32) -> ! {
        process::exit(code);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Oak: missing filename!");
        process::exit(1);
    }

    let filename = args[1].clone();

    let program: String = fs::read_to_string(&filename).unwrap_or_else(|_err| {
        eprintln!("Oak: file not found!");
        process::exit(1);
    });

    let prgm_config = Config::new(program, Rc::new(StdInterface {}));

    match prgm_config.run() {
        Ok(val) => println!("Oak - result: {:#?}", &*val),
        Err(e) => eprintln!("Oak - interpreter error: {}", e.reason),
    };
}