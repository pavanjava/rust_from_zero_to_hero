# Ownership in Rust: Concept

Ownership is Rust's most unique feature, and it has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.

## Why ownership exists

Every program has to release the memory it no longer needs. Languages solve this in one of three ways.

| Approach | How memory is freed | Example languages |
| --- | --- | --- |
| Garbage collection | A runtime process finds and frees unused memory | Java, Python, Go |
| Manual memory management | The programmer allocates and frees memory explicitly | C, C++ |
| Ownership model | The compiler inserts the cleanup, based on a set of rules checked at compile time | Rust |

## Comparison

| | Pros | Cons |
| --- | --- | --- |
| Garbage collection | Error free\*<br>Faster write time | No control over memory<br>Slower and unpredictable runtime performance<br>Larger program size |
| Manual memory management | Control over memory<br>Faster runtime<br>Smaller program size | Error prone<br>Slower write time |
| Ownership model | Control over memory<br>**Error free\***<br>Faster runtime<br>Smaller program size | Slower write time<br>Learning curve (fighting with the borrow checker) |

\* Error free refers to memory errors such as use after free, double free, and dangling pointers. It does not mean the program is free of logic bugs.

The ownership model keeps the runtime benefits of manual memory management and the safety of garbage collection. The price is paid by the programmer at write time: the compiler rejects code that breaks the ownership rules.

## Ownership rules

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### Rule 1: every value has an owner

```rust
let s = String::from("hello");
```

The variable `s` is the owner of the string value.

### Rule 2: only one owner at a time

```rust
let s1 = String::from("hello");
let s2 = s1;
// println!("{}", s1);  // error: borrow of moved value: `s1`
```

Assigning `s1` to `s2` moves ownership. `s2` is now the only owner, and `s1` can no longer be used.

### Rule 3: the value is dropped when the owner goes out of scope

```rust
{
    let s = String::from("hello");
    // s is valid here
}
// s is out of scope; its memory has been freed
```

No explicit free call is written. The compiler inserts the cleanup at the closing brace.

## Key terms

| Term | Meaning |
| --- | --- |
| Owner | The variable responsible for a value |
| Scope | The region of code, usually a `{ }` block, in which a variable is valid |
| Move | Transfer of ownership from one variable to another |
| Drop | Automatic release of a value's memory when its owner goes out of scope |
| Borrow checker | The part of the compiler that enforces the ownership rules |

## Author

Pavan Kumar Mantha