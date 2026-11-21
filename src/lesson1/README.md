# rust_from_zero_to_hero

Practice examples for learning Rust, one lesson at a time.

## Structure

```
rust_from_zero_to_hero/
├── Cargo.toml
└── src/
    └── lession1/
        └── main.rs    # Hello, world!
```

## Requirements

- Rust and Cargo, installed through [rustup](https://rustup.rs)

## Lessons

| Lesson | Topic |
| --- | --- |
| lession1 | Toolchain basics: install, rustc, cargo |

## Lesson 1: Toolchain basics

### Install Rust

Install on Linux, Mac, or Windows through [rustup](https://rustup.rs), then verify:

```
rustc --version
cargo --version
```

Open the offline documentation:

```
rustup docs
```

### Compile with rustc

Compile a single source file and run the executable it produces:

```
cd src/lession1
rustc main.rs
./main
```

### Initialize, compile and run with cargo

Create a new project:

```
cargo new <project_name>
```

Compile, then run the binary from `target/debug`:

```
cargo build
./target/debug/main
```

Or compile and run in one step:

```
cargo run
```

Since the source file is not at the default `src/main.rs`, `Cargo.toml` points to it with a `[[bin]]` section:

```toml
[[bin]]
name = "main"
path = "src/lession1/main.rs"
```

### Check without producing an executable

Confirm the program compiles, faster than a full build:

```
cargo check
```

### Create a production-ready executable

Build with optimizations. The binary is written to `target/release/main`:

```
cargo build --release
```

## Command summary

| Command | Purpose |
| --- | --- |
| `rustc main.rs` | Compile a single file |
| `cargo new` | Initialize a project |
| `cargo build` | Compile a debug build into `target/debug` |
| `cargo run` | Compile and run |
| `cargo check` | Check that the code compiles, no executable |
| `cargo build --release` | Compile an optimized build into `target/release` |

## Author

Pavan Kumar Mantha