# Major language revision ideas

## Concepts

Everything is a function/list. List in that it can be represented as a list,
function in that the first element of the list defines how to process the rest.

Every list can itself be used as a function, by using it as the first element
of another list (i.e. function call). Really a list is just a function that
returns a function that adds a given index to a memory address and returns the
value stored there.

All memory is just lists. So functional programming but in the context of
global lists.

``
(mem new_identifier (char 'a'))
``

Trying to integrate a high level functional approach with low level usage of how
a computer actually works.

Will need to use shorthand:
1 is actually (int 1), 'test' is actually (str (char 't')) etc...

## Typing

Functins are considered types, because the output
of a given function has a given meaning/structure.
Basic types are `char`, `int`, etc. The actual data returned by these functions
is just binary data, but since it comes from a function we as the programmer
(or the compiler) knows what it represents. So types just keep track of what
functions returned a given value.
For example, (char 'a') returns 61, which is the same binary data as (int 61)
returning 61, but a different type.
For a custom function that returns a char (as opposed to an int): we can
technically follow the calls to see what basic type it came from:

```
(fn test (block
  (char test)
)
(test 'e')
```

In this case the return value is of type (test (char)), a kind of linked list of
types/functions.

Must also support aliases: any function can return a structure of any other type.
This is also how we can support typed arrays.

Basic types like char just evaluate to themselves:
`'a'` compiles to `(ch 61)` compiles/evaluates to `61` (1b of binary data).

For getting and setting values in a list, use the following. Note the exact
same notation would be used for getting and setting values anywhere in memory
(.text, .data, heap, stack, etc.), each of which could be represented as a
separate list

```
(<list> <index>)           // get
(<list> <index> <val>)     // set
```

Defining your own lists/types/functions:
Basically defining rules on how to process the remaining elements of a list.

```
(fn <alias> <stack frame mappings> <list (rules) to evaluate>)
// uses stack frame mappings, inserts code to manage stack.
(sfm <arg1> <arg2> ...) // maps arg names to stack frame location
(asserttype <name> <type>) // compile-time assertion to expect this type of argument
```

## Compile-time vs. Runtime

At compile time, we use the type or alias of a function/list for type
checking. That is, when evaluating arguments to a function call, we first check
types, then evaluate.
At run time, everything is evaluated as untyped binary data (the most reduced
list form).

Eager evaluation at compile time: everything that can be evaluated is
evaluated. So string literals are processed down to their binary equivalents,
if they can be assigned in the .data segment they are, etc.
Macros are therefore supported, but only as completely ordinary functions that
operate on the special `text` list, which contains the text of the program
(probably in a kind of AST format). Since their arguments are therefore fully
defined at compile time, they are run at compile time. Nothing to stop runtime
text manipulation though.

At runtime, lazy evaluation, so only evaluate function calls when their
data is needed. So full fn list stored in memory, to be evaluated when needed
(also when it will actually have data in the stack to work on).

## References

Need some kind of reference; function calls can't include copies as the first
element. Any identifier refers to a list held in memory.

## Design decisions

 - Pipe operator (i.e. symbol that always refers to the value of the last evaluated
   statement in a block)? Yea or nay?
 - Maybe a kind of ownership - each list can either be modified by a single actor,
   or read by many
 - Backend: LLVM? x64 assembly? Custom, with small runtime? Note operations on the text
   segment at would be a whole lot easier with a custom backend/runtime.
 - Lisp quotes (e.g. (quote test))?
 - Rust or C?

```
(main testIdent 'str')
```