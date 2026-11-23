# Lesson 5: Control Flow

How Rust makes decisions and repeats work: `if` and `else`, `if` as an expression, nested conditions, logical operators, `match` on an enum, and `loop`.

## Run

With the `[[bin]]` path in `Cargo.toml` pointing to this lesson's `main.rs`:

```
cargo run
```

## Concepts

### if and else

The condition must be a `bool`. Rust does not treat numbers or other values as true or false, and the condition needs no parentheses.

```rust
let number = 6;
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

### if as an expression

`if` evaluates to a value, so it can sit on the right side of `let`. Both branches must produce the same type.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

The branch values have no semicolon, following the expression rule from lesson 4.

### Nested conditions

An `if` can be placed inside any branch of another `if`.

```rust
let number = 15;
if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is odd");
    if number > 10 {
        println!("{} is greater than 10", number);
    } else {
        println!("{} is not greater than 10", number);
    }
}
```

### Logical operators

| Operator | Meaning | True when |
| --- | --- | --- |
| `&&` | AND | Both sides are true |
| `\|\|` | OR | At least one side is true |
| `!` | NOT | The value is false |

Both `&&` and `||` short-circuit: the right side is evaluated only when the left side has not already decided the result.

```rust
let a: i32 = 10;
let b: i32 = 5;
let c: i32 = 20;

if a > b && b > c {
    println!("a is greater than b and b is greater than c");
} else {
    println!("The condition && is not met");
}
```

Here `a > b` is true but `b > c` is false, so the `else` branch runs. With `||` in place of `&&`, the first branch would run.

### match with an enum

`match` compares a value against a list of patterns and runs the arm that fits. It must cover every variant, and like `if`, it evaluates to a value.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_of_coin(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- Removing any arm raises a compile error, because the `match` would no longer be exhaustive.
- The enum and the function are both defined inside `main`. Rust allows items to be declared inside a function body; they are then visible only within it.

### loop

`loop` repeats its body forever until a `break`. A value placed after `break` becomes the value of the whole loop.

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

The loop runs until `counter` reaches 10, then ends with the value 20.

### Shadowing in this lesson

`number` is declared three times with `let` (6, then 5, then 15). Each declaration shadows the one before it, as covered in lesson 2.

## Expected output

```
condition was false
The value of number is: 5
number is odd
15 is greater than 10
The condition && is not met
value_of_coin: 25
The result is 20
```

## Compiler warnings

| Warning | Reason |
| --- | --- |
| Unused variable `penny` | It is created but never passed to `value_of_coin` |
| Variants `Nickel` and `Dime` are never constructed | Only `Penny` and `Quarter` are created |

## Author

Pavan Kumar Mantha