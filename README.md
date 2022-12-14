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

# Getting Started

1. Clone the **Yellowstone repo**: ```git clone https://github.com/Gearhartlove/yellowstone```

2. Navigate to the "yellowstone" folder

3. Run the REPL and write some code with: ```cargo run```

# Docs

## Typing 
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
nil
```

## Operations
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

Compare two expressions using the `and` or `or` operator. 

```js
true or false // evaluates to false
true and true // evaluates to true
true or false // evaluates to true
false or false // evaluates to false
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

## Statements

**if else**

Branching logical if-else statements similar to other languages. Each condition is followed by a block. If a condition evaluates to true, the statement block directly following is executed and the other conditions are not evaluated.

```js
var x = 2;
if (x == 1) {
    print("x is 1");
} 
else if (x == 2) {
    print("x is 2");
}
else {
    print("x is NOT one or 2"); 
}

output: x is 2
```

**while**

Execute a block of code until a condition is no longer true. 

```js
var num = 1;
while (num != 3) {
  num = num + 1;
}
print num;

output: 3
```

**print**

Convection print statement seen in most programming languages. Notably, there are no parenthesis surrounding the printed content. 

```js
print "Hello World!";

output: Hello World!
```

**assert_eq**

Used to compare the value of two expressions to eachother. Requeries that both expressions evaluate to the same type. If both values evaluate to the same value, the execution of the program continues. Otherwise, the program execution is stopped and an error is returned. 

```js
assert_eq(true, 3==3); // assert statement passes, execution continues.
assert_eq(false, 3==3); // error returned: RUNTIME_ASSERT_ERROR.
```

## Variables

**assignment**

Data can be captured using variables. Every variable has an identifier and an assigned value. The value is of any type in the yellowstone programming language. Variables are defined using the var keyword.

```js
var x = 10.0
var y = "Hello World"
var z = nil
```

**variable shadowing**

The name of a variable can be temporarily overwritten with a different variable assignment.

```js
var foo = "first";

{
  var foo = "second";
  assert_eq(foo, "second");
} 

assert_eq(foo, "first"); 
```
