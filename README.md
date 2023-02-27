# rust_learning
Trying out rust for the first time.

My primary language is C (not C++), so this will be from the perspective of someone who is used to C and it's conventions.
GCC, GDB, and Make are all tools that I use often when making a project in C. I may reference these tools in this file when comparing the C experience to the Rust experience.

I am following "the book" on the rust website: https://doc.rust-lang.org/book/

#### Notes
### rustc
Rust compiler. Basically rust's version of gcc.

### Cargo
cargo is essentially rust's version of make.
cargo new: creates a new project.

cargo build: Compiles and builds executable.
cargo build --release: Fully optimizes code. Good for benchmarking and final programs. Compiles slower than 'cargo build'.
cargo check: Compiles but does not build executable. Good for quick compile checks.
cargo run: Compiles and runs the program. Most often used.

#### Things about rust that I like

* Using ! distinctly marks that macros do not act the same as functions. 
  * This is something that takes students a long time to learn in C and requires me to be carefult about using very specific language, like "invoke" and "expand" instead of "call" to help them distinguish.
  * Making invokation syntactically different from calling helps to distinguish macros from functions.



