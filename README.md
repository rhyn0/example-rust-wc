# Example WC in Rust

Saw this as a fun revisit to a simple CLI tool to get more used to using Rust.

## Usage

1. Get the binary
    1. Build it using `cargo build -r`
1. Run the binary with some input text `oxwc Cargo.toml`

## Code Quality

Using [pre-commit](https://pre-commit.com) I enforce some code quality on code change.

Checks:
    - Formatting - using rustfmt
    - Linting - using clippy
