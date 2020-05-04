use std::collections::HashMap;
use std::rc::Rc;
use super::errors::*;
use super::expressions::*;

#[derive(Debug)]
pub struct Function {
  pub parameters: Vec<String>,
  pub body: Box<dyn Expression>,
}

impl PartialEq for Function {
  fn eq(&self, other: &Self) -> bool {
    self.parameters == other.parameters
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

pub struct Scope<'a> {
  map: HashMap<String, Rc<Value>>,
  parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
  pub fn new(parent: Option<&'a Scope<'a>>) -> Self {
    let is_global = parent.is_none();

    let mut scope = Scope {
      map: HashMap::new(),
      parent,
    };

    if is_global {
      scope.map.insert(String::from("true"), Rc::new(Value::Boolean(true)));
      scope.map.insert(String::from("false"), Rc::new(Value::Boolean(false)));
    }

    scope
  }

  pub fn get(&self, id: &str) -> Result<Rc<Value>, EvalError> {
      match self.map.get(id) {
        Some(val) => Ok(Rc::clone(val)),
        None => {
          match &self.parent {
            Some(parent_scope) => parent_scope.get(id),
            None => Err(EvalError::new("Reference error: name is not defined"))
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