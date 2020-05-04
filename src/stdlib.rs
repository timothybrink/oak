use crate::expressions::Expression;
use crate::classes::*;
use crate::errors::*;
use std::fmt::Debug;
use std::rc::Rc;

// NativeExpression struct, used to create stdlib functions more easily.

pub struct NativeExpression<F>
where 
  F: Fn(&Scope<'_>) -> Result<Rc<Value>, EvalError>
{
  pub function: F,
}

impl<F> NativeExpression<F>
where 
  F: Fn(&Scope<'_>) -> Result<Rc<Value>, EvalError>
{
  pub fn new(f: F) -> NativeExpression<F> {
    NativeExpression {
      function: f,
    }
  }
}

impl<F> Expression for NativeExpression<F>
where
  F: Fn(&Scope<'_>) -> Result<Rc<Value>, EvalError>
{
  fn evaluate(&self, scope: &Scope<'_>, _pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    (self.function)(scope)
  }
}

impl<F> Debug for NativeExpression<F>
where
  F:  Fn(&Scope<'_>) -> Result<Rc<Value>, EvalError>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("NativeExpression")
  }
}

pub fn insert_stdlib(scope: &mut Scope) {
  let fns = vec![
    ("print", Function {
      parameters: vec!["input".to_string()],
      body: Box::new(NativeExpression::new(|scope| {
        println!("{}", *scope.get("input")?);
        Ok(Rc::new(Value::Null))
      }))
    }),
    ("+", Function {
      parameters: vec!["v1".to_string(), "v2".to_string()],
      body: Box::new(NativeExpression::new(|scope| {
        let v1 = match *scope.get("v1")? {
          Value::Number(num) => num,
          _ => return Err(EvalError::new("+ only valid for numbers!".to_string())),
        };
        let v2 = match *scope.get("v2")? {
          Value::Number(num) => num,
          _ => return Err(EvalError::new("+ only valid for numbers!".to_string())),
        };
        Ok(Rc::new(Value::Number(v1 + v2)))
      }))
    })
  ];

  for (fn_name, fn_obj) in fns {
    scope.set(fn_name.to_string(), Rc::new(Value::Function(fn_obj)))
  }
}