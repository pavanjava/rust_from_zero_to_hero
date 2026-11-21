# Lesson 2: Variables, Mutability, Shadowing and Constants

How Rust declares variables, when their values can change, how shadowing works, and how constants differ from variables.

## Structure

```
rust_from_zero_to_hero/
├── Cargo.toml
└── src/
    └── lession2/
        └── main.rs    # Variables!
```

## Run

```
cargo run --bin lesson2
```

## Concepts

### Immutable by default

A variable declared with `let` cannot be reassigned.

```rust
let x: i32 = 5;
// x = 6;  // error: cannot assign twice to immutable variable
```

### Mutability with `mut`

Adding `mut` allows the value to change.

```rust
let mut y = 10;
y = 20;  // allowed
```

### The type is fixed

A variable keeps the type it was declared with. `mut` allows a new value, never a new type. This holds for every variable, mutable or not.

```rust
let mut y = 10;
// y = "Hello to new variable";  // error: expected integer, found `&str`
```

### Shadowing

Declaring a variable again with `let` creates a new variable that hides the earlier one.

In an inner scope, the shadow ends with the block:

```rust
let z = 30;
{
    let z = 10;  // shadows z only inside this block
}
// z is 30 again here
```

In the same scope, the new variable can be built from the old one:

```rust
let z = z + 20;  // z is now 50
```

### Shadowing vs `mut`

| | `mut` | Shadowing |
| --- | --- | --- |
| Keyword | `let mut` once, then plain assignment | `let` every time |
| Can change the value | Yes | Yes |
| Can change the type | No | Yes |
| Result | Same variable, new value | A new variable with the same name |

### Constants

```rust
const MAX_POINTS: u32 = 100_000;
```

1. A constant can never be mutable; `mut` is not allowed with `const`.
2. A constant cannot be shadowed by a `let` binding.
3. A constant must be declared with a data type.
4. Its value must be known at compile time.
5. By convention the name is written in `UPPER_SNAKE_CASE`.

The underscore in `100_000` is a visual separator and does not change the value.

## Expected output

```
The value of x is 5
The value of y is 10
The value of z is 30
The value of z is 10
The value of z is 50
The value of MAX_POINTS is 100000
```

## Errors seen in this lesson

| Code | Error |
| --- | --- |
| Reassigning an immutable variable | `cannot assign twice to immutable variable` |
| Assigning a string to an integer variable | `expected integer, found &str` |

## Author

Pavan Kumar Mantha