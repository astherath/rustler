# Rustler

`rustler` is a CLI tool written in Rust to easily find, index, and display special breadcrumb comments. 

This includes comments that include any of (case insensitive):

- `TODO`
- `FIXME`
- `XXX`
- `NOTE`

## Installation

`cargo install` should take care of all dependencies, run `cargo build --release` to build the binary.



## Usage

```
rustler 1.0
Rustles files for TODO and FIXME comments

USAGE:
rustler <filename> <context>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

ARGS:
<filename>    Sets the input file to rustle
<context>     Tells rustler how many files of surrounding context to return for special lines
```

