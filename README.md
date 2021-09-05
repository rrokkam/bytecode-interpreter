# bytecode-interpreter

Work-in-progress interpreter loosely based on the second half of [Crafting Interpreters]
(https://craftinginterpreters.com/). Unlike his language `clox`, this interpreter is implemented
in Rust.

- To analyze compile time:
  - run `cargo +nightly build -Z timings --release` to generate cargo-target.html
  - open it in your browser: `open cargo-target.html` from this directory.