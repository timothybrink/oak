use crate::common::*;
use crate::expressions::*;
use std::fmt::Debug;
use std::rc::Rc;

// NativeExpression struct, used to create stdlib functions more easily.

pub struct NativeExpression<F>
where
    F: Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>,
{
    pub function: F,
}

impl<F> NativeExpression<F>
where
    F: Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>,
{
    pub fn new(f: F) -> NativeExpression<F> {
        NativeExpression { function: f }
    }
}

impl<F> Expression for NativeExpression<F>
where
    F: Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>,
{
    fn evaluate(&self, scope: Rc<Scope>, _pipe_val: Rc<Value>) -> Result<Rc<Value>, EvalError> {
        (self.function)(scope)
    }
}

impl<F> Debug for NativeExpression<F>
where
    F: Fn(Rc<Scope>) -> Result<Rc<Value>, EvalError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NativeExpression")
    }
}

pub fn insert_stdlib(scope: &mut Scope) {
    let fns = vec![
        // Print function
        (
            "print",
            Function {
                parameters: vec!["input".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    scope.sysint.log(scope.get("input")?);
                    Ok(Rc::new(Value::Null))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // def function
        (
            "def",
            Function {
                parameters: vec!["identifier".to_string(), "value".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let identifier = scope.get("identifier")?;
                    let value = scope.get("value")?;

                    if let Value::Function(id_obj) = &*identifier {
                        let scope = match &id_obj.closure {
                            Some(s) => Rc::clone(s),
                            None => {
                                return Err(EvalError::new("Scope could not be found!".to_string()))
                            }
                        };

                        match &*id_obj.call(Vec::new())? {
                            Value::StringType(id_name) => {
                                scope.set(id_name.to_string(), Rc::clone(&value));
                                Ok(value)
                            }
                            _ => Err(EvalError::new(
                                "The first argument of def must be a function returning a string!"
                                    .to_string(),
                            )),
                        }
                    } else {
                        Err(EvalError::new(
                            "The first argument of def must be a function returning a string!"
                                .to_string(),
                        ))
                    }
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // Add function
        (
            "+",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let v1 = &*scope.get("v1")?;
                    let v2 = &*scope.get("v2")?;
                    Ok(v1 + v2)
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // multiply function
        (
            "*",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let v1 = &*scope.get("v1")?;
                    let v2 = &*scope.get("v2")?;
                    Ok(v1 * v2)
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // div function
        (
            "div",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    if let Value::Number(n1) = &*scope.get("v1")? {
                        if let Value::Number(n2) = &*scope.get("v2")? {
                            return Ok(Rc::new(Value::Number(n1 / n2)));
                        }
                    }
                    Err(EvalError::new(
                        "div requires numbers as arguments!".to_string(),
                    ))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // power function
        (
            "**",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    if let Value::Number(n1) = &*scope.get("v1")? {
                        if let Value::Number(n2) = &*scope.get("v2")? {
                            return Ok(Rc::new(Value::Number(n1.powf(*n2))));
                        }
                    }
                    Err(EvalError::new(
                        "** requires numbers as arguments!".to_string(),
                    ))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // sqrt function
        (
            "sqrt",
            Function {
                parameters: vec!["val".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    if let Value::Number(num) = &*scope.get("val")? {
                        let result = num.sqrt();
                        if result.is_nan() {
                            Ok(Rc::new(Value::Null))
                        } else {
                            Ok(Rc::new(Value::Number(result)))
                        }
                    } else {
                        Err(EvalError::new(
                            "sqrt requires a number as argument!".to_string(),
                        ))
                    }
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        (
            "=",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let v1 = &*scope.get("v1")?;
                    let v2 = &*scope.get("v2")?;
                    Ok(Rc::new(Value::Boolean(v1 == v2)))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        (
            "<",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    if let Value::Number(n1) = &*scope.get("v1")? {
                        if let Value::Number(n2) = &*scope.get("v2")? {
                            return Ok(Rc::new(Value::Boolean(n1 < n2)));
                        }
                    }
                    Err(EvalError::new(
                        "< requires numbers as arguments!".to_string(),
                    ))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        (
            ">",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    if let Value::Number(n1) = &*scope.get("v1")? {
                        if let Value::Number(n2) = &*scope.get("v2")? {
                            return Ok(Rc::new(Value::Boolean(n1 > n2)));
                        }
                    }
                    Err(EvalError::new(
                        "> requires numbers as arguments!".to_string(),
                    ))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        (
            "<=",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    if let Value::Number(n1) = &*scope.get("v1")? {
                        if let Value::Number(n2) = &*scope.get("v2")? {
                            return Ok(Rc::new(Value::Boolean(n1 <= n2)));
                        }
                    }
                    Err(EvalError::new(
                        "<= requires numbers as arguments!".to_string(),
                    ))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        (
            ">=",
            Function {
                parameters: vec!["v1".to_string(), "v2".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    if let Value::Number(n1) = &*scope.get("v1")? {
                        if let Value::Number(n2) = &*scope.get("v2")? {
                            return Ok(Rc::new(Value::Boolean(n1 >= n2)));
                        }
                    }
                    Err(EvalError::new(
                        ">= requires numbers as arguments!".to_string(),
                    ))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // if function
        (
            "if",
            Function {
                parameters: vec![
                    "condition".to_string(),
                    "then".to_string(),
                    "else".to_string(),
                ],
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
                        Err(EvalError::new(
                            "if condition argument must evaluate to boolean!".to_string(),
                        ))
                    }
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // type function
        (
            "type",
            Function {
                parameters: vec!["obj".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let obj = scope.get("obj")?;

                    let type_str = match *obj {
                        Value::Number(_) => "number",
                        Value::StringType(_) => "string",
                        Value::Boolean(_) => "boolean",
                        Value::Function(_) => "function",
                        Value::Null => "null",
                    };
                    Ok(Rc::new(Value::StringType(type_str.to_string())))
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // for function; function gets evaluated with arguments index and accumulator
        (
            "for",
            Function {
                parameters: vec!["array".to_string(), "function".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let array = scope.get("array")?;
                    let function = scope.get("function")?;

                    if let Value::Function(arr_obj) = &*array {
                        if !arr_obj.parameters.contains(&"index".to_string()) {
                            Err(EvalError::new("for requires a valid array".to_string()))
                        } else if let Value::Function(fn_obj) = &*function {
                            let mut index = 0;
                            let mut prev: Rc<Value> = Rc::new(Value::Null);
                            loop {
                                let index_val = Rc::new(Value::Number(index as f64));
                                let element = arr_obj.call(vec![Rc::clone(&index_val)])?;
                                if let Value::Null = &*element {
                                    break;
                                }
                                prev = fn_obj.call(vec![index_val, prev])?;
                                index += 1;
                            }
                            Ok(prev)
                        } else {
                            Err(EvalError::new(
                                "for requires a function as the second argument".to_string(),
                            ))
                        }
                    } else {
                        Err(EvalError::new("for requires a valid array".to_string()))
                    }
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // foreach function; function gets evaluated with arguments item and accumulator
        (
            "foreach",
            Function {
                parameters: vec!["array".to_string(), "function".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let array = scope.get("array")?;
                    let function = scope.get("function")?;

                    if let Value::Function(arr_obj) = &*array {
                        if !arr_obj.parameters.contains(&"index".to_string()) {
                            Err(EvalError::new("foreach requires a valid array".to_string()))
                        } else if let Value::Function(fn_obj) = &*function {
                            let mut index = 0;
                            let mut prev: Rc<Value> = Rc::new(Value::Null);
                            loop {
                                let index_val = Rc::new(Value::Number(index as f64));
                                let element = arr_obj.call(vec![Rc::clone(&index_val)])?;
                                if let Value::Null = &*element {
                                    break;
                                }
                                prev = fn_obj.call(vec![element, prev])?;
                                index += 1;
                            }
                            Ok(prev)
                        } else {
                            Err(EvalError::new(
                                "foreach requires a function as the second argument".to_string(),
                            ))
                        }
                    } else {
                        Err(EvalError::new("foreach requires a valid array".to_string()))
                    }
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // exit function
        (
            "exit",
            Function {
                parameters: vec!["code".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let code = scope.get("code")?;
                    let code = match *code {
                        Value::Number(n) => n as i32,
                        _ => 0,
                    };
                    scope.sysint.exit(code)
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
        // findIndex function; returns index of given item in given array
        (
            "findIndex",
            Function {
                parameters: vec!["array".to_string(), "item".to_string()],
                body: Rc::new(NativeExpression::new(|scope| {
                    let array = scope.get("array")?;
                    let item = scope.get("item")?;

                    if let Value::Function(arr_obj) = &*array {
                        let mut index = 0;
                        loop {
                            let element =
                                arr_obj.call(vec![Rc::new(Value::Number(index as f64))])?;
                            // if element equals item, return index
                            if element == item {
                                break Ok(Rc::new(Value::Number(index as f64)));
                            }
                            // end of array; didn't find; return null
                            if let Value::Null = &*element {
                                break Ok(Rc::new(Value::Null));
                            }
                            index += 1;
                        }
                    } else {
                        Err(EvalError::new(
                            "the first argument of findIndex must be an array!".to_string(),
                        ))
                    }
                })),
                closure: Some(Rc::new(Scope::new_global(Rc::clone(&scope.sysint)))),
            },
        ),
    ];

    for (fn_name, fn_obj) in fns {
        scope.set(fn_name.to_string(), Rc::new(Value::Function(fn_obj)))
    }
}

pub fn get_prelude() -> Vec<Rc<dyn Expression>> {
    vec![
        // ordered set mapping function; returns a function that takes elements of
        // the first array and returns the corresponding element of the second array.
        // The parsed Oak below corresponds exactly to the following line of code:
        // (def .'osm' /arr1 arr2 ./item .(arr2 (findIndex arr1 item)))
        Rc::new(FunctionExpression {
            function: Rc::new(IdentifierExpression {
                name: "def".to_string(),
            }),
            arguments: vec![
                Rc::new(LiteralExpression {
                    value: Rc::new(Value::Function(Function {
                        parameters: vec![],
                        body: Rc::new(LiteralExpression {
                            value: Rc::new(Value::StringType("osm".to_string())),
                            closure: false,
                        }),
                        closure: None,
                    })),
                    closure: true,
                }),
                Rc::new(LiteralExpression {
                    value: Rc::new(Value::Function(Function {
                        parameters: vec!["arr1".to_string(), "arr2".to_string()],
                        body: Rc::new(LiteralExpression {
                            value: Rc::new(Value::Function(Function {
                                parameters: vec!["item".to_string()],
                                body: Rc::new(FunctionExpression {
                                    function: Rc::new(IdentifierExpression {
                                        name: "arr2".to_string(),
                                    }),
                                    arguments: vec![Rc::new(FunctionExpression {
                                        function: Rc::new(IdentifierExpression {
                                            name: "findIndex".to_string(),
                                        }),
                                        arguments: vec![
                                            Rc::new(IdentifierExpression {
                                                name: "arr1".to_string(),
                                            }),
                                            Rc::new(IdentifierExpression {
                                                name: "item".to_string(),
                                            }),
                                        ],
                                    })],
                                }),
                                closure: None,
                            })),
                            closure: true,
                        }),
                        closure: None,
                    })),
                    closure: true,
                }),
            ],
        }),
    ]
}
