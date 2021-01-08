# Oak

### Interpreted Functional Programming

Oak is my attempt at a programming language based on a single core concept, which every
part of the language then fits into. The approach was partly inspired by languages like
Haskell and Lisp, and the central idea of Oak is that everything is a composition and
assembly of functions and expressions. Keep in mind that Oak is really more of a proof
of concept than a serious language, however.

Try it out in your browser at [my website](https://www.timothybrink.dev/oak).

## Concepts

In Oak, everything you write is an expression. There are four types of expressions:

 1. Literal expressions, which provide a way to define values (e.g. a string, number, or
function).
 2. Identifier expressions, which represent a value in memory.
 3. Block expressions, which are a series of other expressions.
 4. Function expressions, which consist of some other expression that is evaluated with
    inputs and produces an output (function application).
 
In this way, the entire semantics of Oak essentially consists of three things: data,
function definition, and function application.

## Syntax
### Literal expressions

There are six types of literal expressions: function, boolean, numeric, string,
array, and null.

 - Function literals have the following syntax:
   `/paramName1 paramName2 .<expression  body>`. The `/` may be ommitted for functions
   with no parameters. Immediately following the `.` is an expression to be evaluated
   when the function is called, which will have the arguments injected into its scope.
   Function literals are closures, so the body of a function can access the scope in
   which the function literal is defined.
 - Booleans are just special identifiers: `true` and `false`.
 - Numeric literals are written as usual: `-10`, `0.1`.
 - String literals can be written with either single or double quotes, and can span
   multiple lines. For example `'foo'` and `"bar"`. Backslashes to escape characters are
   supported.
 - Array literals are written with square brackets. Elements are separated by spaces, as
   follows: `[1 2 3]`. Elements can be expressions. Arrays are not a special data type,
   although it may look that way: an array literal is just a simple way to create a
   function that takes an index as input and returns the element associated with that
   index. Indexes out of range return null.
 - The null value is represented by another special identifier, `null`.

### Identifiers

Identifiers are written as follows: `variableName`. When the interpreter
evaluates an identifier it simply retrieves the associated value from memory.
Identifiers can be made up of any non-reserved set of non-whitespace characters, where
reserved characters are usually just characters utilized in other syntax. There is one
special identifier: `^`. See the block expression section for what it represents.

### Block Expressions

Block expressions are denoted by curly braces (`{}`), and consist of a list of
expressions separated by whitespace. They can span multiple lines. In a block
expression, the `^` identifier is associated with the value of the previous expression
evaluated, or `null` if used in the first expression in the list. Block expressions
evaluate to the result of their last expression.

### Function Expressions

Function expressions (function application) are denoted with soft brackets enclosing a
list of expressions separated by whitespace. The first expression must evaluate to a
function, and any later ones are arguments to the function. For example,
`(print 'Hello World!')`.

## Defining Identifiers (Function Definition)

Identifiers are associated with values via the special `def` function, which takes two
arguments. The first argument must be a function literal which returns a string. The
string will be the name of the identifier. The second argument is any expression, which
will be evaluated and the resulting value stored in memory.

The reason that the first argument must be a function literal returning a string
(`.'identifier'`) is that the `def` function is not a pure function. Since every part of
Oak is supposed to be pure, it requires a bit of a hack to modify the scope in which
`def` was called. This is, admittedly, the language's only real departure from an
entirely pure language, but an entirely pure language is so difficult to work with that
I thought I'd compromise on this one point, for the sake of usability. Technically,
there are a few impure built-ins as well (`print` and `exit`), but they only modify
state external to the program.

As for the second argument, any expression is valid. The result of evaluating that
expression will be stored in memory, whether it is a constant value or a function. There
is no real difference between values or functions as far as the `def` function or the
memory is concerned.

## Examples

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

Closure demonstration:
```
(def .'externalConstant' 100)

(def .'function' .{
  externalConstant
})

(function)
```
Output: `100`

Recursion:
```
(def .'!' /x .(if (= x 1) .1 .(* x (! (+ x -1)))))

(! 5)
```
Output: `120`

For more syntax and usage examples, see the examples directory.

## Built-ins

Built-in functions (like `+`, `print`, and so on) are overridable, as the interpreter
considers them to be ordinary functions defined in the topmost scope. See src/stdlib.rs
for definitions of the various functions.

Also, in addition to built-ins defined in Rust, there is support for built ins defined
in (parsed) Oak: see the end of src/stdlib.rs for an example.

## Usage

The Oak interpreter is written in Rust. To obtain the source code,
`git clone https://github.com/timothybrink/oak.git`. To build it, use
`cargo build`. The resulting binary takes a single argument: the oak file to run.
Also, because the crate has a single binary, you can `cargo install` it if you like.

### WASM

The Oak interpreter can also be compiled to WASM using the [wasm-pack tool](https://rustwasm.github.io/wasm-pack/).
A simple `wasm-pack build` should work, to build it for a bundler (use the wasm-pack `--target` option
for other JS targets). Note that the Rust code expects a log_oak
function exposed at a global level to take print messages from Oak.