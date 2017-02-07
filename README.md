# rustex

Exercises to learn the Rust programming language.

## Getting Started

Prerequisite: Simply visit https://www.rustup.rs/
and follow the instructions to install Rust.

For each exercise, the goal is to ensure that both
`cargo run` and `cargo test` execute without error:

```sh
[0001_hello]$ cargo run
   Compiling 0001_hello v0.1.0 (file:///home/ben/Projects/rustex/0001_hello)
    Finished debug [unoptimized + debuginfo] target(s) in 0.29 secs
     Running `target/debug/0001_hello`
Hello Samwise
Hello Marvin
[0001_hello]$ cargo test
   Compiling 0001_hello v0.1.0 (file:///home/ben/Projects/rustex/0001_hello)
    Finished debug [unoptimized + debuginfo] target(s) in 0.33 secs
     Running target/debug/deps/0001_hello-72158781518ace74

running 1 test
test test_hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## Learning Materials

- http://rust-lang.github.io/book/
- http://rustbyexample.com/