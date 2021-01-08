use crate::common::Value;
use std::{process, rc::Rc};

#[cfg(not(target_arch = "wasm32"))]
pub fn log(msg: Rc<Value>) {
    println!("{}", msg);
}

#[cfg(target_arch = "wasm32")]
pub fn log(msg: Rc<Value>) {
    crate::log_oak(&msg.to_string());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn exit(code: i32) -> ! {
    process::exit(code);
}

#[cfg(target_arch = "wasm32")]
pub fn exit(_code: i32) -> ! {
    loop {}
}
