#[derive(Debug)]
pub struct EvalError {
  pub reason: &'static str,
}

impl EvalError {
  pub fn new(reason: &'static str) -> EvalError {
    EvalError { reason }
  }
}