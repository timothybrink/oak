use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Display;
use std::fmt::Debug;
use std::ops::Add;
use std::cell::RefCell;
use super::errors::*;
use super::expressions::*;

#[derive(Debug, Clone)]
pub struct Function {
  pub parameters: Vec<String>,
  pub body: Rc<dyn Expression>,
  pub closure: Option<Rc<Scope>>,
}

impl Function {
  pub fn set_closure(&mut self, scope: Rc<Scope>) {
    self.closure = Some(scope);
  }

  pub fn call(&self, arguments: Vec<Rc<Value>>) -> Result<Rc<Value>, EvalError> {
    // Given values, call the function.

    // This bit of matching is necessary for the Rc::clone below.
    let closure_scope = match &self.closure {
      Some(s) => s,
      None => return Err(EvalError::new("Functions must have a closure scope!".to_string())),
    };
    let closure_scope = Rc::clone(&closure_scope);

    // Then, create a function scope with the values of the arguments
    let fn_scope = Scope::new(Some(closure_scope));

    let mut args = arguments.iter();
    for param_name in self.parameters.iter() {
      match args.next() {
        Some(val) => fn_scope.set(param_name.clone(), Rc::clone(val)),
        None => continue,
      };
    }

    // Then, evaluate the function body. Note that for now, pipe is given null
    // in a new function evaluation.
    Ok(self.body.evaluate(Rc::new(fn_scope), Rc::new(Value::Null))?)
  }
}

impl PartialEq for Function {
  fn eq(&self, _other: &Self) -> bool {
    false
  }
}

#[derive(Debug, PartialEq)]
pub enum Value {
  Number(f64),
  StringType(String),
  Boolean(bool),
  Function(Function),
  Null,
}

impl Add for &Value {
  type Output = Rc<Value>;

  fn add(self, rhs: Self) -> Rc<Value> {
    let output = match self {
      Value::Number(num1) => {
        if let Value::Number(num2) = rhs {
          Value::Number(num1 + num2)
        } else {
          Value::Null
        }
      },
      Value::StringType(str1) => {
        if let Value::StringType(str2) = rhs {
          Value::StringType(str1.clone() + str2)
        } else {
          Value::Null
        }
      },
      _ => Value::Null,
    };
    Rc::new(output)
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    let val = match self {
      Value::Number(num) => num.to_string(),
      Value::StringType(st) => st.clone(),
      Value::Boolean(b) => b.to_string(),
      Value::Function(_) => "Function".to_string(),
      Value::Null => "Null".to_string(),
    };
    write!(f, "{}", val)?;
    Ok(())
  }
}

#[derive(Debug)]
pub struct Scope {
  map: RefCell<HashMap<String, Rc<Value>>>,
  parent: Option<Rc<Scope>>,
}

impl Scope {
  pub fn new(parent: Option<Rc<Scope>>) -> Self {
    let is_global = parent.is_none();

    let mut hash_map = HashMap::new();

    if is_global {
      hash_map.insert(String::from("true"), Rc::new(Value::Boolean(true)));
      hash_map.insert(String::from("false"), Rc::new(Value::Boolean(false)));
      hash_map.insert(String::from("null"), Rc::new(Value::Null));
    }

    let scope = Scope {
      map: RefCell::new(hash_map),
      parent,
    };

    scope
  }

  pub fn get(&self, id: &str) -> Result<Rc<Value>, EvalError> {
      match self.map.borrow().get(id) {
        Some(val) => Ok(Rc::clone(val)),
        None => {
          match &self.parent {
            Some(parent_scope) => parent_scope.get(id),
            None => Ok(Rc::new(Value::Null))
          }
        }
      }
  }

  pub fn set(&self, id: String, val: Rc<Value>) {
    self.map.borrow_mut().insert(id, val);
  }
}

pub struct StringIterator<'a> {
  next_value: Option<char>,
  iter: Box<dyn Iterator<Item = char> + 'a>,
}

impl<'a> Iterator for StringIterator<'a> {
  type Item = char;

  fn next(&mut self) -> Option<char> {
    let val = self.next_value;
    self.next_value = self.iter.next();
    val
  }
}

impl<'a> StringIterator<'a> {
  pub fn new<'b>(string: &'b String) -> StringIterator {
    let mut iter = Box::new(string.chars());
    let next_value = iter.next();
    
    StringIterator {
      next_value,
      iter,
    }
  }

  pub fn preview(&self) -> Option<char> {
    self.next_value
  }
}