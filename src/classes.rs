use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Display;
use std::fmt::Debug;
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
  Array(Vec<Value>),
  Boolean(bool),
  Function(Function),
  Null,
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    let val = match self {
      Value::Number(num) => num.to_string(),
      Value::StringType(st) => st.clone(),
      Value::Array(_) => "[]".to_string(),
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
  map: HashMap<String, Rc<Value>>,
  parent: Option<Rc<Scope>>,
}

impl Scope {
  pub fn new(parent: Option<Rc<Scope>>) -> Self {
    let is_global = parent.is_none();

    let mut scope = Scope {
      map: HashMap::new(),
      parent,
    };

    if is_global {
      scope.map.insert(String::from("true"), Rc::new(Value::Boolean(true)));
      scope.map.insert(String::from("false"), Rc::new(Value::Boolean(false)));
      scope.map.insert(String::from("null"), Rc::new(Value::Null));
    }

    scope
  }

  pub fn get(&self, id: &str) -> Result<Rc<Value>, EvalError> {
      match self.map.get(id) {
        Some(val) => Ok(Rc::clone(val)),
        None => {
          match &self.parent {
            Some(parent_scope) => parent_scope.get(id),
            None => Ok(Rc::new(Value::Null))
          }
        }
      }
  }

  pub fn set(&mut self, id: String, val: Rc<Value>) {
    self.map.insert(id, val);
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