use super::classes::*;
use super::errors::*;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

mod parsers;

// ################################################################
// #                       EXPRESSION TRAIT                       #
// ################################################################
pub trait Expression: fmt::Debug {
  fn evaluate(&self, scope: &Scope<'_>, pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError>;
}

// ################################################################
// #                      LITERAL EXPRESSION                      #
// ################################################################
#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpression {
  value: Rc<Value>,
}

impl LiteralExpression {
  pub fn new(iter: &mut StringIterator) -> Result<LiteralExpression, EvalError> {
    match iter.preview() {
      Some('0'..='9')        => Ok(LiteralExpression { value: parsers::number_parser(iter)? }),
      Some('\'') | Some('"') => Ok(LiteralExpression { value: parsers::string_parser(iter)? }),
      Some('/') | Some('.')  => Ok(LiteralExpression { value: parsers::function_parser(iter)? }),
      Some(_)                => Err(EvalError::new("Unknown character!")),
      None                   => Err(EvalError::new("End of string reached")),
    }
  }
}

impl Expression for LiteralExpression {
  fn evaluate(&self, _scope: &Scope, _pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    Ok(Rc::clone(&self.value))
  }
}

// ################################################################
// #                    IDENTIFIER EXPRESSION                     #
// ################################################################
#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierExpression {
  name: String,
}

impl IdentifierExpression {
  pub fn new(iter: &mut StringIterator) -> Result<IdentifierExpression, EvalError> {
    let mut name = String::new();
  
    loop {
      let next_char = match iter.preview() {
        Some(val) => val,
        None => break,
      };
      if next_char.is_whitespace() || "(){}[]./".contains(next_char) {
        break;
      }
      name.push(next_char);
      iter.next();
    }
  
    Ok(IdentifierExpression{ name })
  }
}

impl Expression for IdentifierExpression {
  fn evaluate(&self, scope: &Scope, pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    if self.name == "^" {
      Ok(pipe_val)
    } else {
      Ok(scope.get(&self.name)?)
    }
  }
}

// ################################################################
// #                       BLOCK EXPRESSION                       #
// ################################################################
#[derive(Debug)]
pub struct BlockExpression {
  expressions: Vec<Box<dyn Expression>>,
}

impl BlockExpression {
  pub fn new(iter: &mut StringIterator) -> Result<BlockExpression, EvalError> {
    let is_program_block = match iter.preview() {
      Some('{') => {
        // consume opening bracket
        iter.next();
        false
      },
      Some(_) => true,
      None => false,
    };
    let mut expressions: Vec<Box<dyn Expression>> = Vec::new();

    loop {
      let next_char = match iter.preview() {
        Some(val) => val,
        None => break
      };
      
      // consume whitespace
      if next_char.is_whitespace() {
        iter.next();
        continue;
      } else if !is_program_block && next_char == '}' {
        // consume
        iter.next();
        break;
      } else {
        expressions.push(parsers::generic(iter)?);
      }
    }

    Ok(BlockExpression {
      expressions,
    })
  }
}

impl Expression for BlockExpression {
  fn evaluate(&self, scope: &Scope<'_>, pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    let block_scope = Scope::new(Some(scope));

    let mut val = pipe_val;

    for expr in self.expressions.iter() {
      val = expr.evaluate(&block_scope, Rc::clone(&val))?;
    }

    // Loop through expressions, return result of the last one.
    Ok(val)
  }
}

// ################################################################
// #                     FUNCTION EXPRESSION                      #
// ################################################################
#[derive(Debug)]
struct FunctionExpression {
  identifier: IdentifierExpression,
  arguments: Vec<Box<dyn Expression>>,
}

impl FunctionExpression {
  pub fn new(iter: &mut StringIterator) -> Result<FunctionExpression, EvalError> {
    // Consume opening parenthesis
    iter.next();

    // First is the identifier.
    let identifier = IdentifierExpression::new(iter)?;

    // Then arguments:
    let mut arguments: Vec<Box<dyn Expression>> = Vec::new();

    loop {
      let next_char = match iter.preview() {
        Some(val) => val,
        None => return Err(EvalError::new("End of function expression not found!"))
      };

      // consume whitespace
      if next_char.is_whitespace() {
        iter.next();
        continue;
      } else if next_char == ')' {
        iter.next();
        break;
      } else {
        arguments.push(parsers::generic(iter)?);
      }
    }

    Ok(FunctionExpression {
      identifier,
      arguments,
    })
  }
}

impl Expression for FunctionExpression {
  fn evaluate(&self, scope: &Scope, pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
    // First, get the function object
    let fn_obj = self.identifier.evaluate(scope, Rc::clone(&pipe_val))?;

    let fn_obj = match &*fn_obj {
      Value::Function(obj) => obj,
      _ => return Err(EvalError::new("Identifier does not reference a valid function!"))
    };

    // Then, evaluate the arguments
    let mut args = self.arguments.iter().map(|arg_expr| {
      arg_expr.evaluate(scope, Rc::clone(&pipe_val))
    });

    // Then, create a function scope with the values of the arguments
    let mut fn_scope = Scope::new(Some(scope));

    for param_name in fn_obj.parameters.iter() {
      match args.next() {
        Some(Ok(val)) => fn_scope.set(param_name.clone(), val),
        Some(Err(e)) => return Err(e),
        None => continue,
      };
    }

    // Then, evaluate the function body. Note that for now, pipe is given null
    // in a new function evaluation.
    Ok(fn_obj.body.evaluate(&fn_scope, Rc::new(Value::Null))?)
  }
}

// ################################################################
// #                             TESTS                            #
// ################################################################
#[cfg(test)]
mod tests {
  use crate::classes::*;
  use std::rc::Rc;

  #[test]
  fn parses_numerics() {
    let s = &String::from("100.0");
    let mut s = StringIterator::new(s);
    let exp = super::LiteralExpression::new(&mut s).unwrap();
    assert_eq!(exp, super::LiteralExpression { value: Rc::new(Value::Number(100.0)) })
  }

  #[test]
  fn parses_strings() {
    let s = &String::from("'it\\'s a \"test\"\\\\'  ");
    let mut s = StringIterator::new(s);
    let exp = super::LiteralExpression::new(&mut s).unwrap();
    assert_eq!(exp, super::LiteralExpression {
      value: Rc::new(Value::StringType(String::from("it's a \"test\"\\")))
    })
  }

  #[test]
  fn parses_functions() {
    let s = &"/test a b .'string'W".to_string();
    let mut s = StringIterator::new(s);
    let exp = super::LiteralExpression::new(&mut s).unwrap();
    println!("{:?}", exp);
  }

  #[test]
  fn parses_identifiers() {
    let s = &"+test)tes ".to_string();
    let mut s = StringIterator::new(s);
    let exp = super::IdentifierExpression::new(&mut s).unwrap();
    assert_eq!(exp, super::IdentifierExpression{ name: String::from("+test") })
  }

  #[test]
  fn parses_function_calls() {
    let s = &"(test 'a b c' (b) /arg c e .{c e})".to_string();
    let mut s = StringIterator::new(s);
    let exp = super::FunctionExpression::new(&mut s).unwrap();
    println!("{:#?}", exp);
  }

  #[test]
  fn parses_blocks() {
    let s = &"{10 'test' (fn a b) (def .test /a b c .{body})}".to_string();
    let mut s = StringIterator::new(s);
    let exp = super::BlockExpression::new(&mut s).unwrap();
    println!("{:#?}", exp);
  }
}