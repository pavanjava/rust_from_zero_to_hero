# Lesson 4: Functions

How to define and call functions, pass parameters, return values, and the difference between statements and expressions.

## Run

With the `[[bin]]` path in `Cargo.toml` pointing to this lesson's `main.rs`:

```
cargo run
```

## Concepts

### Defining and calling a function

A function is declared with `fn`, a name in `snake_case`, parentheses, and a body in curly braces. It is called by its name followed by parentheses.

```rust
fn integers_in_rust() {
    // body
}

fn main() {
    integers_in_rust();
}
```

- `main` is the entry point of the program.
- Functions can be defined before or after `main`. The order of definitions does not matter.
- A function that returns nothing returns the unit type `()`.

In this lesson, the integer and numeral system code from lesson 3 is moved out of `main` into two functions, `integers_in_rust` and `numerical_system`.

### Parameters

Parameters are listed inside the parentheses. Every parameter must have a type annotation; the compiler does not infer it.

```rust
fn compute(num: i32) -> i32 {
    return num * 2
}
```

The value passed in the call, `compute(10)`, is the argument.

### Return values

The return type is declared after `->`. A function can return a value in two ways:

| Way | Example | Note |
| --- | --- | --- |
| `return` keyword | `return num * 2` | Explicit; also used to exit a function early |
| Last expression | `num * 2` | No `return` and no semicolon; the idiomatic form |

Both versions of `compute` return the same value.

### Statements and expressions

| | Statement | Expression |
| --- | --- | --- |
| What it does | Performs an action | Evaluates to a value |
| Produces a value | No | Yes |
| Example | `let x = 5;` | `x + 1` |

A block in curly braces is itself an expression. Its value is the last expression inside it:

```rust
let y = {
    let x = 5;
    x + 1
};
```

Here `y` is 6, and `x` exists only inside the block.

### The semicolon rule

Adding a semicolon to the last line turns the expression into a statement. The block then has no value to give and evaluates to `()`.

| Last line of the block | Value of `y` |
| --- | --- |
| `x + 1` | 6 |
| `x + 1;` | `()` |

With the semicolon, `println!("y = {}", y)` no longer compiles, because `()` cannot be printed with `{}`.

The same rule applies to a function body: a final line of `num * 2;` in a function declared to return `i32` raises a mismatched types error.

## Expected output

```
small_number: -127
big_number: -123456789012345678
small_number: -127
big_number: -123456789012345678
decimal: 98222
hex: 255
octal: 63
binary: 240
byte: 65
The computed response = 20
y = 6
```

## Author

Pavan Kumar Mantha