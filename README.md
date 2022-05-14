# blazefuck

A blazingly-fast (interactive) Brainfuck interpreter, written in Rust.<br><br>

## Description

A tiny, efficient [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter, with a REPL for easy, on-the-fly evaluation

Brainfuck is a simple esoteric language with a minimal subset of commands, you can read more [here](https://en.wikipedia.org/wiki/Brainfuck#:~:text=Brainfuck%20is%20an%20esoteric%20programming%20language%20created%20in%201993%20by%20Urban%20M%C3%BCller.).
<br><br>
## Getting Started

### Dependencies

* Install the [rustup](https://rustup.rs/) toolchain for your system.

### Installing

* Add Cargo's binary directory to your path environment variables<br>The rustup installer should do this for you, but if not, it should be located at `~/.cargo/bin` (Linux) or `%USERPROFILE\.cargo\bin` (Windows).
* Simply run `cargo install blazefuck` at a terminal and it'll be installed from crates.io.

### Executing program

* Type `blazefuck` at a prompt

```
$ blazefuck
blazefuck 1.0.0 on windows, run with "-h" or "--help" for more information.
Use "cells" to show the cell stack and "exit" to exit the interpreter.
>>> _
```
<br>
You can then use the normal Brainfuck commands, as follows:

|       |                                                                                                                                                                                   |
|-------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **>** | Increment the data pointer (to point to the next cell to the right).                                                                                                              |
| **<** | Decrement the data pointer (to point to the next cell to the left).                                                                                                               |
| **+** | Increment (increase by one) the byte at the data pointer.                                                                                                                         |
| **-** | Decrement (decrease by one) the byte at the data pointer.                                                                                                                         |
| **.** | Output the byte at the data pointer.                                                                                                                                              |
| **,** | Accept one byte of input, storing its value in the byte at the data pointer.                                                                                                      |
| **[** | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. |
| **]** | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. |

Also `cells` and `exit` will show the current cell stack and exit the REPL, respectively.<br><br>
* Alternatively a source file can be specified with `blazefuck [FILE]`, some examples can be found [here](examples).
```
$ blazefuck hello.bf
Hello World!

$ _
```

* Some flags can be specified as follows:
```
-d, --debug     Shows the cell stack after every command
-s, --strict    Activates strict mode
```

* Strict mode has some key differences, as opposed to normal mode, some programs will not run well with it enabled.<br>It:
    * Disallows access of any cells not between #1 and #30000, normal mode wraps around to the beginning or end.
    * Disallows cell data being incremented or decremented past 0-255, normal mode also wraps around.<br><br>

## Building from source

* Clone this repository
```
$ git clone https://github.com/poopsicles/blazefuck
```

* Switch to the newly created directory 
```
$ cd ./blazefuck
```

* Compile using cargo
```
$ cargo build
```

* Cargo will grab the required dependencies and create the binary at `./target/debug/blazefuck`<br><br>

## Version History

* 1.0
    * Initial release<br><br>

## License

This project is licensed under the MIT License, more details [here](LICENSE).