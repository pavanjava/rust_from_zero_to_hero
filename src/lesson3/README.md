# Lesson 3: Data Types

Rust's built-in scalar and compound types, plus the two custom types: structs and enums.

| Category | Types |
| --- | --- |
| Scalar | Integers, floating-point numbers, booleans, characters |
| Compound | Tuples, arrays |
| Custom | Structs, enums |

## Run

With the `[[bin]]` path in `Cargo.toml` pointing to `src/lesson3/main.rs`:

```
cargo run
```

## Scalar types

### Integers

| Length | Signed | Unsigned |
| --- | --- | --- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

- Signed types hold negative and positive values; unsigned types hold zero and positive values only.
- `u8` ranges from 0 to 255, `i8` from -128 to 127.
- `isize` and `usize` match the machine architecture: 64 bits on a 64-bit system.
- An integer without a type annotation defaults to `i32`.

```rust
let small_number: u8 = 255;
let big_number: i128 = -123456789012345678;
```

### Numeral systems

| Numeral system | Description | Example | Decimal value |
| --- | --- | --- | --- |
| Decimal | Base-10, common form | `98_222` | 98222 |
| Hexadecimal | Base-16, prefixed with `0x` | `0xff` | 255 |
| Octal | Base-8, prefixed with `0o` | `0o77` | 63 |
| Binary | Base-2, prefixed with `0b` | `0b1111_0000` | 240 |
| Byte (`u8` only) | ASCII character, prefixed with `b` | `b'A'` | 65 |

Whatever form the literal is written in, `{}` prints it in decimal.

### Floating-point numbers

Two types: `f32` and `f64`. A float without a type annotation defaults to `f64`.

```rust
let x = 2.0;
let y: f32 = 3.0;
```

Rust does not mix float types in arithmetic. Because `x` is later used with `y` (an `f32`), the compiler infers `f32` for `x` as well. It would be `f64` only if nothing else constrained it.

### Numeric operations

| Operation | Operator | Example | Result |
| --- | --- | --- | --- |
| Addition | `+` | `2.0 + 3.0` | 5 |
| Subtraction | `-` | `2.0 - 3.0` | -1 |
| Multiplication | `*` | `2.0 * 3.0` | 6 |
| Division | `/` | `2.0 / 3.0` | 0.6666667 |
| Remainder | `%` | `43 % 5` | 3 |

### Booleans

The `bool` type has two values, `true` and `false`. It can be declared implicitly or explicitly, used directly as an `if` condition, and negated with `!`.

```rust
let t = true;
let f: bool = false;
let not_true = !t;
```

### Characters

A `char` is written in single quotes, takes 4 bytes, and holds one Unicode scalar value, so it is not limited to ASCII.

```rust
let c = '𝕏';
```

`.chars()` iterates over Unicode scalar values, not over what a reader sees as letters. The string `"नमस्ते 🙏"` yields 8 chars:

| # | Char | Code point | Note |
| --- | --- | --- | --- |
| 1 | न | U+0928 | Letter |
| 2 | म | U+092E | Letter |
| 3 | स | U+0938 | Letter |
| 4 | ् | U+094D | Combining sign (virama) |
| 5 | त | U+0924 | Letter |
| 6 | े | U+0947 | Combining vowel sign |
| 7 | (space) | U+0020 | Space |
| 8 | 🙏 | U+1F64F | Emoji |

The combining signs are printed on their own lines, separated from the letters they belong to.

## Compound types

### Tuples

A fixed-length group of values that can have different types. Elements are accessed by index with a dot.

```rust
let tup: (i32, f64, char) = (500, 6.4, 'x');
let five_hundred = tup.0;
```

### Arrays

A fixed-length group of values that all share one type. Elements are accessed by index with square brackets, and `.iter()` loops over them.

```rust
let arr = [1, 2, 3, 4, 5];
let first = arr[0];

for element in arr.iter() {
    println!("element: {}", element);
}
```

### Tuple vs array

| | Tuple | Array |
| --- | --- | --- |
| Element types | Can differ | All the same |
| Length | Fixed | Fixed |
| Access | `tup.0` | `arr[0]` |
| Type of the example | `(i32, f64, char)` | `[i32; 5]` |

## Custom types

### Structs

A struct groups named fields under one type. Fields are read with a dot.

```rust
struct Person {
    name: String,
    age: u8,
}

let person = Person {
    name: String::from("John"),
    age: 30,
};
```

### Enums

An enum is a type whose value is exactly one of a fixed set of variants. `match` handles each variant and must cover all of them.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

let light = TrafficLight::Yellow;

match light {
    TrafficLight::Yellow => println!("Light Yellow"),
    TrafficLight::Green => println!("Light Green"),
    TrafficLight::Red => println!("Light Red"),
}
```

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
x = 2, y = 3
sum = 5
difference = -1
product = 6
quotient = 0.6666667
remainder = 3
t = true, f = false
t is true
not_true = false
c = 𝕏
```

Then the 8 chars of `"नमस्ते 🙏"`, one per line, followed by:

```
The value of five_hundred is: 500
The value of six_point_four is: 6.4
The value of x_char is: x
first: 1, second: 2
element: 1
element: 2
element: 3
element: 4
element: 5
name is John
age is 30
Light Yellow
```

## Notes

- The signed pair of `_small_number` and `_big_number` shadows the unsigned pair before anything is printed, so the signed values appear twice and 255 is never shown.
- A leading underscore in a variable name tells the compiler not to warn when the variable is unused. It is part of the name.
- The compiler warns that the variants `Red` and `Green` are never constructed, because the code only creates `Yellow`.

## Author

Pavan Kumar Mantha