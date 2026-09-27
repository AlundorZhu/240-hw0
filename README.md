# Homework 5: Introduction to Rust (Optional)

**Note: This homework is optional. It will be worth a small amount of extra credit (5 points).**

The homework is self-contained, and should give you a small taste of programming in Rust. If you are interested in learning more about Rust, you can continue to work on this assignment. If you are not interested in Rust, you can skip this assignment.

I originally taught this homework as part of the Rust class at UPenn,
spring 2021.
If you are interested in going further with Rust, all of the original
course material is available [here](https://github.com/upenn-cis198)!
This assignment is HW1 from that course.

## Installation

Rust is very easy to install on most systems!
You should just have to copy and paste the command
from [the Rust website](https://www.rust-lang.org/tools/install).

Rust code is run and built through a tool called Cargo (which is installed automatically by the above).
You should be able to run
```
cargo --version
```
to see your version of Cargo. The main commands you need to know about are the following:
```
cargo run
cargo run --release
cargo test
cargo clippy
cargo fmt
```

The first two are to run your code (in `src/main.rs`), either in debug mode or release mode. The next two are for running your tests and checking your code for common mistakes. The last one is for automatically formatting your code according to the Rust style guide.
You can also use `cargo build` to compile the code without running it.

## Setup

Clone this repository.

You should be be able to run `cargo run` and see no compiler errors or warnings. You should also be able to run `cargo test` and see some failing unit tests.

## Assignment

Please modify files `part1.rs`, `part2.rs`, and `part3.rs` as instructed in each problem to have all code compile, and all tests passing. Testing is an important part of the software development process. Hence we expect to write some unit tests when useful and possible.

Use `cargo test` to compile and run the tests. This may also reveal further compiler errors.

## Clippy and Rustfmt

When writing Rust, before a final submission,
it is best practice to ensure that your `clippy` and `rustfmt`
are happy with your code! (`cargo clippy` and `cargo fmt`)

After everything is implemented for a part, also remove the lines
```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

You are welcome to configure `rustfmt` a bit differently by editing `rustfmt.toml` (e.g. to change max line width). Additionally, if you encounter a case where you think `clippy` or `rustfmt` has it wrong, make a post on Piazza. If I agree, you can disable it for a particular block of code.

## File Structure

The file `main.rs` is what is run when you run `cargo run`, but it also imports a *module* for each part. Every new file creates a new module. Modules are not checked by the compiler unless they're imported. Currently we have in `main.rs`:

```rust
pub mod part1;
// Uncomment these to have Rust compile the other files as well.
// pub mod part2;
// pub mod part3;
```

Once all tests are working on `part1.rs` uncomment as needed to allow the other module to be checked. This way we avoid having errors from part2.rs or part3.rs stop us from running tests or compiling too much of the working code.

You might notice the `pub` keyword everywhere. In each file, this makes the function public so that `main.rs` has access to it, in case you want to run it there. It also has the benefit that once you remove `#![allow(dead_code)]`, you shouldn't get dead code warnings.

## Credits

I originally taught this homework as part of the Rust class at UPenn (CIS 198), spring 2021.
Thanks to the previous instructors for earlier versions of this homework,
especially, [gatowololo (Omar)](https://gatowololo.github.io/).
