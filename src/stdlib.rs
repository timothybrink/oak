use crate::expressions::Expression;
use crate::classes::*;
use crate::errors::*;
use std::fmt::Debug;
use std::rc::Rc;

// NativeExpression struct, used to create stdlib functions more easily.

pub struct NativeExpression<F>
where 
  F: Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>
{
  pub function: F,
}

impl<F> NativeExpression<F>
where 
  F: Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>
{
  pub fn new(f: F) -> NativeExpression<F> {
    NativeExpression {
      function: f,
    }
  }
}

impl<F> Expression for NativeExpression<F>
where
  F: Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>
{
  fn evaluate(&self, scope: Rc<Scope>, _pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    (self.function)(scope)
  }
}

impl<F> Debug for NativeExpression<F>
where
  F:  Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("NativeExpression")
  }
}

pub fn insert_stdlib(scope: &mut Scope) {
  let fns = vec![
    // Print function
    ("print", Function {
      parameters: vec!["input".to_string()],
      body: Rc::new(NativeExpression::new(|scope| {
        println!("{}", *scope.get("input")?);
        Ok(Rc::new(Value::Null))
      })),
      closure: Some(Rc::new(Scope::new(None))),
    }),
    // Add function
    ("+", Function {
      parameters: vec!["v1".to_string(), "v2".to_string()],
      body: Rc::new(NativeExpression::new(|scope| {
        let v1 = &*scope.get("v1")?;
        let v2 = &*scope.get("v2")?;
        Ok(v1 + v2)
      })),
      closure: Some(Rc::new(Scope::new(None))),
    }),
    ("=", Function {
      parameters: vec!["v1".to_string(), "v2".to_string()],
      body: Rc::new(NativeExpression::new(|scope| {
        let v1 = &*scope.get("v1")?;
        let v2 = &*scope.get("v2")?;
        Ok(Rc::new(Value::Boolean(v1 == v2)))
      })),
      closure: Some(Rc::new(Scope::new(None)))
    }),
    // if function
    ("if", Function {
      parameters: vec!["condition".to_string(), "then".to_string(), "else".to_string()],
      body: Rc::new(NativeExpression::new(|scope| {
        let condition = scope.get("condition")?;
        let then_block = scope.get("then")?;
        let else_block = scope.get("else")?;

        if let Value::Boolean(b) = *condition {
          if b {
            // evaluate then block
            if let Value::Function(then_obj) = &*then_block {
              then_obj.call(Vec::new())
            } else {
              Err(EvalError::new("then block must be a function!".to_string()))
            }
          } else {
            // evaluate else block
            if let Value::Function(else_obj) = &*else_block {
              else_obj.call(Vec::new())
            } else {
              Err(EvalError::new("else block must be a function!".to_string()))
            }
          }
        } else {
          Err(EvalError::new("if condition argument must evaluate to boolean!".to_string()))
        }
      })),
      closure: Some(Rc::new(Scope::new(None)))
    })
  ];

  for (fn_name, fn_obj) in fns {
    scope.set(fn_name.to_string(), Rc::new(Value::Function(fn_obj)))
  }
}