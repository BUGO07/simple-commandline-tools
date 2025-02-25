# Simple Commandline Tools

A list of simple but sometimes useful commandline tools, written in rust.


## List of commands

- Todo Manager (todo)
- Quadratic Equation Solver (quadratic)
- Palindrome Checker (palindrome)
- String reverser (reverse)


## Installation

You can download the executable from the releases page or build it yourself. To run the executable you can open a terminal window in the directory of the executable and run
```
./simple-commandline-tools <command> <args>
```
I like to rename the file to "sct" and add it to path, that way its easy to run anytime I open the terminal.
## Building

To build this project on your own you need to have `rust` and `git` installed locally

Clone the project

```bash
  git clone https://github.com/BUGO07/simple-commandline-tools
```

Go to the project directory

```bash
  cd simple-commandline-tools
```

Build with `cargo`

```bash
cargo build --release
```

This will output `target/release/simple-commandline-tools` executable on Linux or one With `.exe` extension on Windows.