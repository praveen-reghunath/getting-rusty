## Commands

### Create new project 

```
cargo new exercise
```
Creates:
* src/main.rs - Entry point with a main() function
* Project can be run directly with cargo run
* Produces an executable when built

```
cargo new exercise --lib
```
The --lib flag in cargo new exercise --lib tells Cargo to create a library project instead of the default binary (executable) project.

Creates:
* src/lib.rs - Library root file
* No main() function by default
* Cannot be run directly with cargo run
* Produces a library (.rlib file) that other projects can depend on
* Includes example tests in src/lib.rs
Libraries are meant to be used by other Rust projects as dependencies, while binary projects create standalone executables. You can run tests in library projects with `cargo test`.


### Run the project
```
cd exercise

cargo run
```

### Format the code 
```
cargo fmt
```

### To check for errors
```
cargo check
```

### Build 
```
cargo build
```

Use `cargo build --release` to produce an optimized release build in `target/release/`.


### Install new packages

```
cargo add rand@0.8.5 trpl@0.2.0
```