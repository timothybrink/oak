# Oak

### Interpreted Functional Programming

Oak is my attempt at a programming language based on a single core concept, which every part of the language then fits into. The approach was partly inspired by languages like Haskell and Lisp, and the central idea of Oak is that everything is a composition and assembly of functions and expressions.

Try it out in your browser at [my website](https://timothybrink.dev/oak).

## Concepts

In Oak, everything you write is an expression. There are four types of expressions: the literal expression, which consists of a value (e.g. string, number, or function); the identifier expression, which represents a value in memory; the block expression, which is a series of other expressions; and the function expression, which is some other expression evaluated with given inputs (i.e. function application). In this way, the entire semantics of Oak consists of three things: data, function definition, and function application.

## Syntax

### Identifiers

Identifiers are written like you'd expect: `variableName`. When the interpreter evaluates an identifier it simply retrieves the associated value from memory. Identifiers can be made up of pretty much any non-reserved set of non-whitespace characters. Reserved characters here are usually just characters used in other syntax. There is one special identifier: `^`. See the block expression section for more info.

### Literal expressions

Base expressions have the most variation in syntax. There are five types of literal expressions: function, boolean, numeric, string, and array.
 - Function literals have the following syntax: `/param1 param2 param3 .<expression body>`. The `/` may be ommitted for functions with no parameters. Immediately following the `.` is an expression to be evaluated when the function is called. It will have the parameters in its scope when the function is called.
 - Booleans are written like identifiers: `true` and `false`.
 - Numeric literals are written like normal: `10` and `0.1`.
 - String literals can be written with either single or double quotes, and can span multiple lines. E.g. `'foo'` and `"bar"`. Backslashes to escape characters are supported.
 - Array literals are written with square brackets. Elements are separated by spaces, e.g. `[1 2 3]`. Elements can be expressions. Arrays are not a special data type in Oak, although it may look that way: an array literal is just a simple way to create a function that takes an index as input and returns the element associated with that index. Indexes out of range return null.
 - There is an additional literal, written as `null`, which represents the null value. This is what is returned when a value does not exist, generally speaking.

### Block Expressions

Block expressions are denoted by curly braces (`{}`), and consist of a list of expressions separated by spaces. They can also span multiple lines. In a block expression, the `^` identifier is associated with the value of the previous expression evaluated, or null when there was no value (e.g. the start of a function or block). When evaluated, the result of the last expression in the list will be the result of the block expression.

### Function Expressions

Function expressions, or function application, is denoted with soft brackets enclosing a list of expressions separated by spaces. The first expression must evaluate to the function to call, and any later ones are the arguments to the function. For example, `(print 'Hello World!')`

### Identifier Declaration (Function Definition)

Identifiers are associated with values via the `def` function, which takes two arguments. The first argument is a function literal returning a string, which will be the name of the identifier associated with the expression, and the second is an expression whose value will be stored in memory.

Note: since the `def` function is not really a pure function, it requires a bit of a hack to get at the scope in which it is called (as usually a function would only have access to the scope in which it was defined). This is the reason for the requirement that you pass it a function literal as the first argument.

As for the last argument, any expression is valid. The result of evaluating that expression will be stored in memory, whether it be a constant value or a function. There is no real difference between values or functions as far as the `def` function or the memory is concerned, however. The only difference is that functions can be called, as described above.

### Examples

Defining a constant:
```
(def .'str' 'hello world')
```

Defining a function:
```
(def .'sum' \arg1 arg2 .{
  (+ arg1 arg2)
})
```

For more syntax and usage examples, see the examples directory.

### Built ins

Built in functions (like `+`, `print`, and so on) are overridable, as the interpreter considers them to be ordinary functions defined in the topmost scope. See src/stdlib.rs for definitions of the various functions.

Also, in addition to built ins defined in Rust, there is support for built ins defined in (parsed) Oak: see the end of src/stdlib.rs for an example.

## Usage

The Oak interpreter is written in Rust. To build it, you should be able to just `git clone` and then either `cargo build --release` to get a binary, or `cargo run <oakfile.oak>` to run an Oak file.