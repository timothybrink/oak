use std::{rc::Rc, process};
use crate::common::Value;

pub fn log(msg: Rc<Value>) {
  println!("{}", msg);
}

pub fn exit(code: i32) -> ! {
  process::exit(code);
}