#[derive(Debug, Clone)]
pub struct EvalError {
    pub reason: String,
}

impl EvalError {
    pub fn new(reason: String) -> EvalError {
        EvalError { reason }
    }
}
