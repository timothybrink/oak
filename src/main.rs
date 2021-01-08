use std::env;
use std::fs;
use std::process;

use oak::Config;

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

    let prgm_config = Config::new(program);

    match prgm_config.run() {
        Ok(val) => println!("Oak - result: {}", &*val),
        Err(e) => eprintln!("Oak - interpreter error: {}", e.reason),
    };
}
