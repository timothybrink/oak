# Major language revision ideas

## Basics

Everything is a list. Programs and data consist of nested scoped lists, of which
computer memory is the topmost layer.

Functional programming is achieved by the evaluation of these lists. Each list,
beginning with the program itself (a list of expressions), is evaluated by
evaluating each of its elements in order.

Functions are simply lists whose elements access other elements in the same
list, and are able to short-circuit evaluation of the list.

A static list has simple data (e.g. `(1 2 3)`). A function is achieved by
replacing simple data (like `1`) with references to the other elements of the
list (`((pl 1) 2 3) -> (2 2 3)`). `pl` is a pointer to the parent list, or the
list that pl was called in. The other essential part of this is the ability for
a list to override the normal evaluation sequence, which would continue through
the other elements and return the resulting list. To do this, we set the value
of the parent list. This short-circuits evalution, and we return out of the
entire parent list with the list given in that function. For example,
`((pset 1) 2 3) -> 1)`, ignoring the final elements of the list.

`(padd q(new_identifier) (char 'a'))`

Trying to integrate a high level functional approach with usage of bare metal
rather than layers and layers of abstraction.

## Typing

Typing is a way of keeping track of what type of data a list contains. Since all
lists are just binary data and do not have a type metadata, we keep track of
types in list tuples. For example, `(char 'a')` is a tuple of a pointer to a
'type constant' (of some kind, not sure yet) and the number 61.

## References

Identifiers refer to indexes in the parent list or super-parent lists, hence
scoping. They contain an indicator of the level of list, and the index in that
list.

For getting and setting values in a list, use the following. Note the exact same
notation would be used for getting and setting values anywhere in memory (.text,
.data, heap, stack, etc.), each of which could be represented as a separate list

`padd` is a shortcut to the current parent list.

```
(get <list> <index>)
(set <list> <index> <val>)
```

Defining your own lists/types/functions: Set a pointer in memory to a list of
expressions. It is necessary to quote the list so as to defer evaluation.

```
(padd (q a_fn) (q
  (print ("hello world"))
  (print (pl 1))
))

(a_fn "test")
-> "hello world"
-> "test"
```

## Compile-time vs. Runtime

At compile time, we use the type or alias of a function/list for type checking.
That is, when evaluating arguments to a function call, we first check types,
then evaluate. At run time, everything is evaluated as untyped binary data (the
most reduced list form).

Eager evaluation at compile time: everything that can be evaluated is evaluated.
So string literals are processed down to their binary equivalents, if they can
be assigned in the .data segment they are, etc. Macros are therefore supported,
but only as completely ordinary functions that operate on the special `text`
list, which contains the text of the program (probably in a kind of AST format).
Since their arguments are therefore fully defined at compile time, they are run
at compile time. Nothing to stop runtime text manipulation though.

At runtime, lazy evaluation, so only evaluate function calls when their data is
needed. So full fn list stored in memory, to be evaluated when needed (also when
it will actually have data in the stack to work on).

## Design decisions

- Pipe operator (i.e. symbol that always refers to the value of the last
  evaluated statement in a block)? Yea or nay?
- Backend: LLVM? x64 assembly? Custom, with small runtime? Note operations on
  the text segment at runtime would be a whole lot easier with a custom backend/runtime.
- Rust or C? Thinking C runtime, but written in Oak as much as possible.