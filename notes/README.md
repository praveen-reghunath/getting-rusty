
# RUST

    * No null pointers

## Installation
Follow steps on [https://rustup.rs/](https://rustup.rs/)

## Tools
    rustc  : the Rust compiler.
    cargo  : the Rust dependency manager and build tool
    rustup : the Rust toolchain installer and updater

[https://crates.io/](https://crates.io/)  - The central hub where the community shares code.

### Cargo

    * Package Manager - It downloads and updates the libraries ( called "crates").
    * Build System - It compiles the code
    * Test Runenr - It runs the units tests and integration tests
    * Project creator - It sets up the standard folder stracture

# Comments

- line `//`
- block `/* ... */`

Doc comments are very useful for big projects that require documentation. When running `rustdoc`, these are the comments that get compiled into documentation. They are denoted by a `///`, and support [Markdown](https://en.wikipedia.org/wiki/Markdown).


### Variables
```rust
let y = 400; // Can't modify
let mut x = 200; // variable can be changed.

```

In Rust variables are immutable by default. This means once you assign a value to a variable, you cannot change it.

Rust allows to declare a new variable with the same name as a previous variable. This is called shadowing.

```rust
let x = 5;
println!("First x: {}", x);
    
let x = x + 1;
println!("Second x: {}", x);
    
let x = x * 2;
println!("Third x: {}", x);
```

## debug

Print variable y

```rust
dbg!(y);
```

### if 

```rust
if x == 0 || x == 1 {
    println!("zero or 1")
} else if x < 100 {
    println!("biggish");
} else {
    println!("huge");
}
```


### match
```rust
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }
```

Like if expressions, match can also return a value;

```rust
fn main() {
    let flag = true;
    let val = match flag {
        true => 1,
        false => 0,
    };
    println!("The value of {flag} is {val}");
}
```

#### Range Patterns (Inclusive)
```rust
match value {
    0..=10 => println!("Between 0 and 10 (inclusive)"),
    11..=20 => println!("Between 11 and 20"),
    _ => println!("Something else"),
}
```

#### Match Guards

```rust
match value {
    x if x > 10 => println!("Greater than 10: {}", x),
    x if x < 0 => println!("Negative: {}", x),
    x => println!("Between 0 and 10: {}", x),
}
```

#### Combining Both cases

```rust
match value {
    0..=5 => println!("Low range"),
    6..=10 => println!("Mid range"),
    x if x > 100 => println!("Very high: {}", x),
    x => println!("High range: {}", x),
}
```

#### Character and Other Types

```rust
match character {
    'a'..='z' => println!("Lowercase letter"),
    'A'..='Z' => println!("Uppercase letter"),
    '0'..='9' => println!("Digit"),
    _ => println!("Other character"),
}
```

## Loops

### for loop
```rust
fn main() {
    for x in 1..5 {
        dbg!(x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }
}
```

### loop
The loop statement just loops forever, until a break.

```rust
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}
```


## Arrays

```rust
fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");
}
```