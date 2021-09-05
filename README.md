# bytecode-interpreter

Work-in-progress interpreter loosely based on `clox`, from [Crafting Interpreters]
(https://craftinginterpreters.com/). Unlike `clox`, this interpreter is implemented in Rust.

Commit messages in this repository start with an intention and risk level, following 
[Arlo's Commit Notation](https://github.com/arlobelshee/ArlosCommitNotation).

- To analyze compile time:
  - run `cargo +nightly build -Z timings --release` to generate cargo-target.html
  - open it in your browser: `open cargo-target.html` from this directory.