# Rust-Basics: Learning Rust by Example

This project is a collection of Rust programs and code examples, organized by chapter, to help you learn and understand the fundamentals and advanced concepts of the Rust programming language. Each file or module demonstrates a specific topic, ranging from basic syntax to advanced features like lifetimes, traits, generics, and object-oriented patterns in Rust.

## Project Structure

- **src/main.rs**: Entry point with comprehensive examples and explanations of Rust basics, including data types, ownership, borrowing, functions, structs, enums, error handling, collections, and more.
- **src/bin/**: Contains separate files for each chapter or topic, such as:
  - `main.rs`: Basic Syntax of Rust Language
  - `chapter_1.rs`: Hello World
  - `chapter_2.rs`: User input, random numbers, and control flow
  - `chapter_3.rs`: Variables, types, and loops
  - `chapter_4.rs`, `chapter_4_2.rs`: Ownership, borrowing, and references
  - `chapter_5.rs`, `chapter_5_2.rs`: Structs and methods
  - `chapter_10.rs`, `chapter_10_2.rs`, `chapter_10_3.rs`: Generics, traits, and lifetimes
  - `chapter_13.rs`, `chapter_13_2.rs`: Closures and iterators
  - `chapter_18.rs`, `chapter_18_2.rs`: Object-oriented patterns and state machines

## How to Install Rust

### Installing rustup on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

This command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

> Rust is installed now. Great!

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```sh
xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the build-essential package.

### Installing rustup on Windows

On Windows, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions for installing Rust. At some point in the installation, you’ll be prompted to install Visual Studio. This provides a linker and the native libraries needed to compile programs. If you need more help with this step, see [https://rust-lang.github.io/rustup/installation/windows-msvc.html](https://rust-lang.github.io/rustup/installation/windows-msvc.html)

The rest of this book uses commands that work in both cmd.exe and PowerShell. If there are specific differences, we’ll explain which to use.

## How to Run

1. **Install Rust**
   Follow the instructions above to install Rust using `rustup`.
2. **Clone this repository** and navigate to the project directory.
3. **Run a specific chapter/example:**
   ```sh
   cargo run --bin chapter_5
   ```
   Replace `chapter_5` with any other file in `src/bin/` to explore different topics.

## Dependencies
- [rand](https://crates.io/crates/rand): Used for random number generation in some examples.

## Author
[![Aditya Thakkar](https://img.shields.io/badge/Aditya%20Thakkar-blue?&style=for-the-badge)](https://github.com/Aditya-A-Thakkar)
