use crate::classes::*;
use crate::errors::EvalError;
use super::*;
use crate::stdlib::NativeExpression;

// Generic expression parser function, used whenever any expression has
// sub expressions to evaluate: decides which expression is there, and calls
// the respective Expression constructor
pub fn generic(iter: &mut StringIterator) -> Result<Rc<dyn Expression>, EvalError> {
  // Assumes no whitespace

  let first_char = match iter.preview() {
    Some(val) => val,
    None => return Err(EvalError::new("End of string; nothing to parse".to_string()))
  };

  if first_char == '{' {
    Ok(Rc::new(BlockExpression::new(iter)?))
  } else if first_char == '(' {
    Ok(Rc::new(FunctionExpression::new(iter)?))
    // Below are all characters that can begin a literal
  } else if first_char.is_digit(10) || ['\'', '"', '[', '/', '.', '-'].contains(&first_char) {
    Ok(Rc::new(LiteralExpression::new(iter)?))
    // Below are all reserved characters that are not covered by previous cases
  } else if !['}', ')', ']', '\\'].contains(&first_char) {
    Ok(Rc::new(IdentifierExpression::new(iter)?))
  } else {
    Err(EvalError::new(format!("Unknown expression type starting with character {}", first_char)))
  }
}

pub fn number_parser(iter: &mut StringIterator) -> Result<Rc<Value>, EvalError> {
  let mut value = String::new();

  loop {
    let this_char = match iter.next() {
      Some(val) => val,
      None => break,
    };
    value.push(this_char);
    let next_char = match iter.preview() {
      Some(val) => val,
      None => break,
    };
    if !next_char.is_digit(10) && next_char != '.' {
      break; // before the non-numeric character is consumed
    }
  }

  println!("{}", value);

  let value: f64 = match value.parse() {
    Ok(val) => val,
    Err(_) => return Err(EvalError::new("Invalid numeric literal!".to_string())),
  };

  Ok(Rc::new(Value::Number(value)))
}

pub fn string_parser(iter: &mut StringIterator) -> Result<Rc<Value>, EvalError> {
  // consume first char
  let first_char = match iter.next() {
    Some(val) => val,
    None => return Err(EvalError::new("End of string reached".to_string())),
  };

  let mut value = String::new();
  let mut escaped = false;

  loop {
    let this_char = match iter.next() {
      Some(val) => val,
      None => {
        return Err(EvalError::new("End of string literal not found!".to_string()))
      }
    };
    if escaped {
      escaped = false;
      value.push(this_char);
    } else if this_char == '\\' {
      escaped = true;
    } else if this_char == first_char {
      break; // here, the closing quote has been consumed
    } else {
      value.push(this_char);
    }
  }

  Ok(Rc::new(Value::StringType(value)))
}

pub fn function_parser(iter: &mut StringIterator) -> Result<Rc<Value>, EvalError> {
  // parse function literal

  // first char, not yet consumed, is either a / or a '.'. If /, parse to the .
  // as parameters, then parse after '.'.

   // Consume. This is safe because we know it is either '/' or '.'
  let first_char = iter.next().unwrap();

  let mut parameters: Vec<String> = Vec::new();

  if first_char == '/' {
    // Parse identifiers, as parameters, until we reach '.'
    let mut current_param = String::new();

    loop {
      match iter.next() {
        Some(' ') => {
          if !current_param.is_empty() {
            parameters.push(current_param.clone());
            current_param.clear();
          } else {
            continue;
          }
        },
        Some('.') => {
          if !current_param.is_empty() {
            parameters.push(current_param.clone());
          }
          break;
        },
        Some(val) => {
          current_param.push(val);
        },
        None => return Err(EvalError::new("Reached end of string while parsing function parameters!".to_string()))
      }
    }
  }

  // Since the '.' has been consumed, and we can only be here if we got it, we
  // can call generic right away.

  let body = generic(iter)?;

  let fn_obj = Function {
    parameters,
    body,
    closure: None,
  };

  Ok(Rc::new(Value::Function(fn_obj)))
}

pub fn array_parser(iter: &mut StringIterator) -> Result<Rc<Value>, EvalError> {
  // consume opening bracket
  iter.next();

  let mut expressions = Vec::new();

  loop {
    match iter.preview() {
      Some(' ') => {
        iter.next();
        continue;
      },
      Some(']') => {
        iter.next();
        break;
      },
      Some(_) => {
        expressions.push(generic(iter)?);
      },
      None => return Err(EvalError::new("Unexpected end of file!".to_string()))
    }
  }

  let fn_body = NativeExpression::new(move |scope| {
    let index = scope.get("index")?;
    let index = if let Value::Number(i) = *index {
      i as usize
    } else {
      return Err(EvalError::new("Arguments to arrays must be numbers!".to_string()))
    };

    let expr = match expressions.get(index) {
      Some(e) => e,
      None => return Ok(Rc::new(Value::Null))
    };
    Ok(expr.evaluate(scope, Rc::new(Value::Null))?)
  });

  let fn_obj = Function {
    parameters: vec!["index".to_string()],
    body: Rc::new(fn_body),
    closure: None,
  };

  Ok(Rc::new(Value::Function(fn_obj)))
}