# Learning Rust <img src="https://www.freecodecamp.org/news/content/images/2021/01/rust-mascot.png" alt="Rust crab icon" width=100px, height=75px>

## Background
This is me trying out rust for the first time.

My primary language is C (not C++), so this will be from the perspective of someone who is familiar with C and it's conventions.

GCC, GDB, and Make are all tools that I use often when making a project in C. I may reference these tools in this file when comparing the C experience to the Rust experience.

I am following "the book" on [the rust website](https://doc.rust-lang.org/book/). The book is where I was recommended to start.

## Notes
### rustc
Rust compiler. Basically rust's version of gcc.

### Cargo
Cargo is essentially rust's version of make. It also acts like a package manager for libraries, it's mirror is [crates.io](https://crates.io).

**cargo new**: Creates a new project.\
**cargo build**: Compiles and builds executable.\
**cargo build --release**: Fully optimizes code. Good for benchmarking and final programs.\
**cargo check**: Compiles but does not build executable. Good for quick "does my code compile?" checks.\
**cargo run**: Compiles and runs the program. Most often used.

## Things about rust that I like

- Using `!` distinctly marks that macros do not act the same as functions.
  - This is something that takes students a long time to learn in C and requires me to be carefult about using very specific language, like "invoke" and "expand" instead of "call" to help them distinguish.
  - Making invokation syntactically different from calling helps to distinguish macros from functions.


+ Cargo is a great packaging tool
  + It is fairly automatic. You don't really need to worry about it all that much.
  + There are only a few commands you need to know to get started.
  + It automatically names your executable file with something identifyable. Technically this is rustc but whatever. I find it better than gcc's `a.out` for everything unless otherwise specified.

+ Shadowing seems a bit dodgy, but hopefully it allows for more clear code. Though I am a bit skeptical of it.
