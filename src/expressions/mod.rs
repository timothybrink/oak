use super::classes::*;
use super::errors::*;
use std::fmt;
use std::fmt::Debug;

mod parsers;

// ################################################################
// #                       EXPRESSION TRAIT                       #
// ################################################################
pub trait Expression: fmt::Debug {
  fn evaluate<'a>(&'a self, scope: &'a mut Scope<'a>) -> Result<&'a Value, EvalError>;
}

// ################################################################
// #                      LITERAL EXPRESSION                      #
// ################################################################
#[derive(Debug, PartialEq)]
pub struct LiteralExpression {
  value: Value,
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
  fn evaluate<'a>(&'a self, scope: &mut Scope) -> Result<&'a Value, EvalError> {
    Ok(&self.value)
  }
}

// ################################################################
// #                    IDENTIFIER EXPRESSION                     #
// ################################################################
#[derive(Debug, PartialEq)]
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
  fn evaluate<'a>(&self, scope: &'a mut Scope) -> Result<&'a Value, EvalError> {
    Ok(scope.get(&self.name)?)
  }
}

// ################################################################
// #                       BLOCK EXPRESSION                       #
// ################################################################
#[derive(Debug)]
struct BlockExpression {
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
  fn evaluate<'a>(&'a self, scope: &'a mut Scope<'a>) -> Result<&'a Value, EvalError> {
    let mut block_scope = Scope::new(Some(scope));

    let mut val = &Value::Null;

    for expr in self.expressions.iter() {
      val = expr.evaluate(&mut block_scope)?;
      // Maybe I need to box the child scope to simplify
      // lifetimes... Might fix it. Another thing is that I'm still not sure
      // about referencing Values in the Result of evaluate()
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
  fn evaluate<'a>(&'a self, scope: &'a mut Scope) -> Result<&'a Value, EvalError> {
    Ok(&Value::Number(0.0))
  }
}

// ################################################################
// #                             TESTS                            #
// ################################################################
#[cfg(test)]
mod tests {
  use crate::classes::*;

  #[test]
  fn parses_numerics() {
    let mut s = StringIterator::new("100.0");
    let exp = super::LiteralExpression::new(&mut s).unwrap();
    assert_eq!(exp, super::LiteralExpression { value: Value::Number(100.0) })
  }

  #[test]
  fn parses_strings() {
    let mut s = StringIterator::new("'it\\'s a \"test\"\\\\'  ");
    let exp = super::LiteralExpression::new(&mut s).unwrap();
    assert_eq!(exp, super::LiteralExpression {
      value: Value::StringType(String::from("it's a \"test\"\\"))
    })
  }

  #[test]
  fn parses_functions() {
    let mut s = StringIterator::new("/test a b .'string'W");
    let exp = super::LiteralExpression::new(&mut s).unwrap();
    println!("{:?}", exp);
  }

  #[test]
  fn parses_identifiers() {
    let mut s = StringIterator::new("+test)tes ");
    let exp = super::IdentifierExpression::new(&mut s).unwrap();
    assert_eq!(exp, super::IdentifierExpression{ name: String::from("+test") })
  }

  #[test]
  fn parses_function_calls() {
    let mut s = StringIterator::new("(test 'a b c' (b) /arg c e .{c e})");
    let exp = super::FunctionExpression::new(&mut s).unwrap();
    println!("{:#?}", exp);
  }

  #[test]
  fn parses_blocks() {
    let mut s = StringIterator::new("{10 'test' (fn a b) (def .test /a b c .{body})}");
    let exp = super::BlockExpression::new(&mut s).unwrap();
    println!("{:#?}", exp);
  }
}