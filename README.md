# bytecode-interpreter

Work-in-progress interpreter loosely based on `clox`, from [Crafting Interpreters]
(https://craftinginterpreters.com/). Unlike `clox`, this interpreter is implemented in Rust.

Commit messages in this repository start with an intention and risk level, following 
[Arlo's Commit Notation](https://github.com/arlobelshee/ArlosCommitNotation).

- To get code coverage using [grcov](https://github.com/mozilla/grcov):
  - `cargo install grcov cargo-watch`
  - `rustup component add llvm-tools-preview`
  - `mkdir -p target/debug/coverage`
  - In your shell config file, add:
    - `export RUSTFLAGS="-Zinstrument-coverage"`
    - `LLVM_PROFILE_FILE="target/debug/coverage/llvm-profile-%p-%m.profraw"`
  - To run tests and update the code coverage when a file is saved, use `cargo watch -x test -s ./watch.sh`
    - To view the coverage in VSCode, use the [Coverage Gutters plugin](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters).

- To analyze compile time:
  - run `cargo +nightly build -Z timings --release` to generate cargo-target.html
  - open it in your browser: `open cargo-target.html` from this directory.
