# Rust: A Practical Crash Course

This guide gets you productive with Rust quickly. It covers setup, core language concepts, and the essential Cargo commands you’ll use every day.

## 1) Setup & Hello World

- Install: Use rustup (installs `rustc`, `cargo`, and toolchains).
- Update: `rustup update`
- New project: `cargo new hello_rust`
- Run: `cd hello_rust && cargo run`

```rs
fn main() {
    println!("Hello, Rust!");
}
```

## 2) Variables, Mutability, Shadowing, Types

```rs
fn main() {
    // Immutable by default
    let x = 5;
    // Mutable
    let mut y: i32 = 10;
    y += 1;

    // Shadowing: re-declare name with a new binding/type
    let x = x + 1;      // new binding
    let x = "now a str"; // even a different type

    // Constants: require type + `const`, compile-time only
    const MAX_POINTS: u32 = 100_000;

    println!("{x}, {y}, {MAX_POINTS}");
}
```

Notes:
- Types you’ll use often: `i32`, `u64`, `f64`, `bool`, `char`, tuples `(i32, bool)`, arrays `[T; N]`, slices `&[T]`.
- String types: `String` (owned, growable) and `&str` (borrowed slice).

## 3) Functions, Expressions, Control Flow

```rs
fn add(a: i32, b: i32) -> i32 { a + b } // expression body, no semicolon

fn main() {
    let sum = add(2, 3);

    if sum > 3 { println!("big"); } else { println!("small"); }

    let label = match sum {
        0 => "zero",
        1 | 2 => "one or two",
        3..=10 => "several",
        _ => "lots",
    };
    println!("{label}");

    for i in 0..3 { println!("i={i}"); }

    let mut n = 3;
    while n > 0 { n -= 1; }

    let val = loop { break 42; }; // loops can yield values
    println!("{val}");
}
```

## 4) Ownership, Borrowing, Slices, Lifetimes

Core rules:
- Each value has a single owner; when the owner goes out of scope, the value is dropped.
- Move semantics by default; `Clone`/`copy` for duplicates.
- Borrowing lets you reference without taking ownership: many immutable borrows `&T` OR one mutable borrow `&mut T` at a time.

```rs
fn len(s: &String) -> usize { s.len() } // immutable borrow
fn push_excl(s: &mut String) { s.push('!') } // mutable borrow

fn main() {
    let mut msg = String::from("hi");

    let l = len(&msg);            // ok: immutable borrow
    push_excl(&mut msg);          // ok: unique mutable borrow

    let slice: &str = &msg[0..2]; // string slice (&str)
    println!("{l}, {slice}, {msg}");
}
```

Minimal lifetime example (usually inferred):

```rs
// Return a reference tied to the longer of inputs
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

## 5) Structs, Enums, Pattern Matching

```rs
struct User { name: String, age: u8 }

impl User {
    fn new(name: &str, age: u8) -> Self { Self { name: name.into(), age } }
    fn is_adult(&self) -> bool { self.age >= 18 }
}

enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { r } => std::f64::consts::PI * r * r,
        Shape::Rect { w, h } => w * h,
    }
}
```

Pattern helpers:
- `if let` to match one variant: `if let Some(x) = maybe { ... }`
- `while let` loops over pattern until it fails.

## 6) Options, Results, Error Handling

- `Option<T>` for “value or none”.
- `Result<T, E>` for fallible operations. Use `?` to bubble up errors.

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_to_string(path: &str) -> io::Result<String> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn parse_number(s: &str) -> Option<i32> { s.trim().parse().ok() }
```

## 7) Collections, Strings

```rs
use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    for x in &v { println!("{x}"); }

    let mut m = HashMap::new();
    m.insert("apples", 3);
    if let Some(qty) = m.get("apples") { println!("{qty}"); }

    let mut s = String::from("Hello");
    s.push_str(", world");
    let hello: &str = &s[0..5];
    println!("{hello} | {s}");
}
```

## 8) Traits, Generics, Impl

```rs
trait Area { fn area(&self) -> f64; }

struct Circle { r: f64 }
impl Area for Circle { fn area(&self) -> f64 { std::f64::consts::PI * self.r * self.r } }

fn max_of<T: PartialOrd + Copy>(a: T, b: T) -> T { if a >= b { a } else { b } }
```

Blanket impls and trait bounds allow composition and zero-cost abstractions.

## 9) Iterators & Closures

```rs
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let sum_sq_evens: i32 = nums
        .iter()           // &i32
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum();

    println!("{sum_sq_evens}");
}
```

Closures capture environment by reference/mutable/owning as needed; traits: `Fn`, `FnMut`, `FnOnce`.

## 10) Modules, Crates, Project Layout

- Binary crate: `src/main.rs` with `fn main()`.
- Library crate: `src/lib.rs` with `pub` items.
- Modules: organize with `mod` and `pub mod`.

```rs
// src/lib.rs
pub mod math {
    pub fn double(x: i32) -> i32 { x * 2 }
}

// src/main.rs
use your_crate::math::double;
fn main() { println!("{}", double(21)); }
```

## 11) Testing, Docs, Tooling

- Unit tests (same file) and integration tests (`tests/`):

```rs
// in src/lib.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds() { assert_eq!(add(2, 2), 4); }
}
```

- Docs: write `///` comments; build and open with `cargo doc --open`.
- Formatting: `cargo fmt`
- Linting: `cargo clippy -- -W clippy::pedantic` (optional pedantic warnings)

## 12) Cargo Essentials

- Create: `cargo new app_name` or `cargo new --lib lib_name`
- Build: `cargo build` (debug) / `cargo build --release`
- Run: `cargo run`
- Test: `cargo test`
- Add dependency: edit `Cargo.toml` under `[dependencies]`

Example `Cargo.toml` snippet:

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
```

## 13) Async (Quick Glimpse)

Rust async uses futures and executors. Common patterns:

```rs
// with an async runtime like tokio
use reqwest::Client;

async fn fetch_status(url: &str) -> Result<u16, reqwest::Error> {
    let resp = Client::new().get(url).send().await?;
    Ok(resp.status().as_u16())
}
```

Concepts: `async fn`, `.await`, `Send`, `Sync`, `!Send` types, pinning. Start with synchronous code; adopt async when IO-bound.

## 14) Common Pitfalls & Tips

- Borrow checker errors usually signal data ownership/borrows need restructuring; clone only when measured and necessary.
- Prefer slices `&[T]`/`&str` in APIs for flexibility.
- Avoid unnecessary `mut`; narrow lifetimes of mutable borrows.
- Use `?` and `thiserror`/`anyhow` (for apps) to simplify error handling.
- Keep modules small and public APIs minimal (`pub` only what’s needed).

## 15) Suggested Learning Path

1. Read “The Rust Book” chapters on ownership, structs/enums, pattern matching.
2. Build 2–3 small CLIs: file processing, HTTP client, todo app.
3. Add tests and benchmarks; practice using iterators and slices.
4. Explore traits/generics with a small library crate.
5. Try async by writing a small web service.

---

Quick reference:
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Build/Run: `cargo build` / `cargo run`
- Test: `cargo test`
- Docs: `cargo doc --open`

Happy Rusting! 🦀

