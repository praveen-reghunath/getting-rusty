
# RUST

    * No null pointers

## Installation
Follow steps on [https://rustup.rs/](https://rustup.rs/).

## Tools
    rustc  : the Rust compiler.
    cargo  : the Rust dependency manager and build tool
    rustup : the Rust toolchain installer and updater

# Comments

- line `//`
- block `/* ... */`

Doc comments are very useful for big projects that require documentation. When running `rustdoc`, these are the comments that get compiled into documentation. They are denoted by a `///`, and support [Markdown](https://en.wikipedia.org/wiki/Markdown).


### Variables
```
let y = 400; // Can't modify
let mut x = 200; // variable can be changed.

```

## debug

Print variable y

```
dbg!(y);
```

### if 

```
if x == 0 || x == 1 {
    println!("zero or 1")
} else if x < 100 {
    println!("biggish");
} else {
    println!("huge");
}
```


### match
```
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

```
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
```
match value {
    0..=10 => println!("Between 0 and 10 (inclusive)"),
    11..=20 => println!("Between 11 and 20"),
    _ => println!("Something else"),
}
```

#### Match Guards

```
match value {
    x if x > 10 => println!("Greater than 10: {}", x),
    x if x < 0 => println!("Negative: {}", x),
    x => println!("Between 0 and 10: {}", x),
}
```

#### Combining Both cases

```
match value {
    0..=5 => println!("Low range"),
    6..=10 => println!("Mid range"),
    x if x > 100 => println!("Very high: {}", x),
    x => println!("High range: {}", x),
}
```

#### Character and Other Types

```
match character {
    'a'..='z' => println!("Lowercase letter"),
    'A'..='Z' => println!("Uppercase letter"),
    '0'..='9' => println!("Digit"),
    _ => println!("Other character"),
}
```

## Loops

### for loop
```
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

``` 
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

```
fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");
}
```