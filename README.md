# ECS 240 Homework 0: Introduction to Rust

**Due: Monday, October 5, 2026 at 11:59pm**

This homework is a coding assignment to help you get set up with Rust
and to provide a Rust tutorial.
This homework will be worth 50 points.
Future homeworks will be problem sets worth 100 points.

## Resources

This homework is generally self-contained.
If you are interested in going further with Rust,
you may want to check out the material for my [Rust course at UPenn](https://github.com/upenn-cis198)
(which includes additional lectures and homeworks),
and my PhD student Muhammad Hassnain has put together a list of Rust resources
[here](https://muhammad-hassnain.github.io/rust/resources/).

I particularly recommend [the Brown version of the Rust book](https://rust-book.cs.brown.edu/) as a tutorial and reference, as it also includes little pop quizzes you can use to check your understanding along the way.

As always, come to office hours or post to Piazza if you get stuck or have any questions!

## Installation

Rust is very easy to install on most systems.
You should just have to copy and paste the command
from [the Rust website](https://www.rust-lang.org/tools/install).

If you are using Windows, I recommend using Windows Subsystem for Linux (WSL).
It's an excellent tool that is well-engineered and widely used by Windows developers in industry. There is a good guide on how to set it up [here](https://learn.microsoft.com/en-us/windows/wsl/install).

If you are editing with VSCode, you will also want to install the [Rust-analyzer VSCode extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer), which is the official language server for Rust.

Rust code is run and built through a tool called Cargo (which is installed automatically by the above).
You should be able to run
```
cargo --version
```
to see your version of Cargo. I'm on Rust 1.98, so to be safe, ensure that your Rust version is at least 1.98. (`rustup update` to update the version of Rust.)
The main commands you need to know about are the following:
```
cargo run
cargo run --release
cargo test
cargo clippy
cargo fmt
```

The first two are to run your code (in `src/main.rs`), either in debug mode or release mode.
Debug mode is used for development, and release mode (much faster) is used in production.
The next two are for running your tests and checking your code for common mistakes. The last one is for automatically formatting your code according to the Rust style guide.
You can also use `cargo build` or `cargo build --release` to compile the code without running it.

## Setup

Clone this repository.

You should be be able to run `cargo run` and see no compiler errors or warnings. You should also be able to run `cargo test` and see some failing unit tests.

## Assignment

Please modify files `part1.rs`, `part2.rs`, and `part3.rs` as instructed in each problem to have all code compile, and all tests passing. Testing is an important part of the software development process. Hence we expect to write some unit tests when useful and possible.

Use `cargo test` to compile and run the tests. This may also reveal further compiler errors.

## Clippy and Fmt

When writing Rust, before a final submission,
it is always best practice to ensure that your `clippy` and `rustfmt`
are happy with your code! (`cargo clippy` and `cargo fmt`)
Clippy is the Rust linter. Rustfmt is the Rust code formatter.
(These can also be configured to run in your code editor, including VSCode with the Rust language extension.)

I require that you run Clippy and Fmt on your code for this HW and address all warnings.
After everything is implemented for a part, remove the following lines:
```rust
#![allow(dead_code)]
#![allow(unused_variables)]
```

These lines tell Clippy to ignore certain warnings, which is useful during development (when we aren't worried about dead code), but we want to check once we have finished and want to ship the code that everything is working properly as intended.

You are welcome to configure `rustfmt` a bit differently by editing `rustfmt.toml`. I have the line width set to 80 characters by default. Additionally, if you encounter a case where you think `clippy` or `rustfmt` has it wrong, make a post on Piazza. If I agree, you can disable it for a particular block of code.

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

## Submission

Submission is via Gradescope.
Please submit your code by uploading **the whole folder, excluding the `target/` and `.git/` folders** via a Zip file.
(I usually find it easiest to copy the entire folder to a new spot, then delete the folders you don't want before zipping. Excluding the `target/` folder is important as that is where all the built code and binaries go, and it can get very bloated for larger projects. This is the same reason why we always add `target/` to `.gitignore`.)

Download your code from the Gradescope website to a new location and run `cargo run` to make sure it worked.

## Credits

Thanks to the previous instructors of CIS 198 at UPenn for earlier versions of this homework,
especially, [gatowololo (Omar)](https://gatowololo.github.io/).
