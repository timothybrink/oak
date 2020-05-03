#[derive(Debug)]
pub struct EvalError {
  reason: &'static str,
}

impl EvalError {
  pub fn new(reason: &'static str) -> EvalError {
    EvalError { reason }
  }
}