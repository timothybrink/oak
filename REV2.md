# Major language revision ideas

## Concepts

Everything is a function/list. List in that it can be represented as a list,
function in that the first element of the list defines how to process the rest.

Every list can itself be used as a function, by using it as the first element
of another list (i.e. function call). Note that the first element defines what
happens when the list as a whole is used as a function; if it is `fn`, the list
is executed, or if it is anything else, the list is indexed into.

## Typing

So types are functions - a type is simply the output of a known function.
Because we know the function, we know the stucture and what we can do with it.
Must also support aliases: any function can return a structure of any other type.
This is also how we can support typed arrays.

Basic types like char just evaluate to themselves:
`'a'` compiles to `(ch 61)` compiles/evaluates to `61` (1b of binary data).

For getting and setting values in a list, use the following. Note the exact
same notation would be used for getting and setting values anywhere in memory
(.text, .data, heap, stack, etc.), each of which could be represented as a separate
list

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