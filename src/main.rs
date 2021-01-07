use std::env;
use std::fs;
use std::process;

use oak::Config;

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

    match oak::run(prgm, config) {
        Ok(val) => println!("Oak - result: {:#?}", &*val),
        Err(e) => eprintln!("Oak - interpreter error: {}", e.reason),
    };
}