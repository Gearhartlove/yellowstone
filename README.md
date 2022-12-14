![Gerald the Bison](/assets/gerald_the_bison.jpg/?raw=true "Title")
# yellowstone 

A simple scripting language inspired by Robert Nystrom's [Crafting Interpreters](http://www.craftinginterpreters.com/). The goal of this project was to write a virtual machine using Rust and investigate programming language design. Over the course of 16 weeks, 4523 lines were written, 80 tests were passed, and basic language features were implemented. These features include: 
* if, else, while statements
* local, global variables
* scopes
* logical operations (<, >, <=, >=, etc...)
* mathematical operations

# WARNING 
Although many language features work, the purpose of this programming language is designed to be educational. It is still in very early stages of development, and parts of code may not function as expected. Documentation is sparse. Please do not attempt to write any serious programs in yellowstone unless you are prepared for many bugs. 

# Design Goals 
* **Test Driven Development**: write many unit and integration tests
* **REPL**: able to write yellowstone in a continious session
* **File**: write code in a '*.ys' file and run the code 
* **Brave**: try to implement good solutions to hard problems 
* **Pure Rust**: source code in 100% Rust
* **No Dependencies**: write all of the code myself 

# Docs

### Typing 
**Floats**

The 32 bit signed `float` type. 
```js
42.0
```
**Booleans**

true and false represent the logical `true` and `false` repectively. 
```js
true
false
```
**Strings**

A UTF-8-encoded, growable `string`. Supports concatenation. 
```js
"Hello World"
```
**Null**

Conventional `null` type.
```js
null
```

### Operations
**Adding**

Add two `strings` or `floats` together with the binary `+` operator. Cannot add different types to each other.

```js
1 + 1 // evaluates to 2
4 * 5 + 3 // evaluates to (4*5)+3 -> 20+3 -> 23
"Hello " + "World" // evaluates to "Hello World"
```

**Subtracting**

Subtract two float expressinos together with the binary `-` operator. 

```js
1 - 1 // evaluates to 0
(4*5) - 3 // evaluates to (4*5)-3 -> 20-3 -> 17
```

**Logical**

Compare two expressions using the `&&` or `||` operator. 

```js
true && false // evaluates to false
true && true // evaluates to true
true || false // evaluates to true
false || false // evaluates to false
```

**Multiplication**

Multiply two logical expressions together with binary `*` operator. 

```js
1 * 8 // evaluates to 8
1 * 2 * 3 // evaluates to 6
```

**Division**

Divide two logical expressions together with binary `/` operator. 

```js
6 / 3  // evaluates to 2
6 / 4  // evaluates to 1.5
6 / 10 // evaluates to 0.6
```


# Getting Started 
