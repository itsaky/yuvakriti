# YuvaKriti

YuvaKriti is a compiled, dynamically typed language with a C-like syntax, written in Rust.

- YuvaKriti is an ongoing personal project, developed while learning Rust, compilers, and virtual machines.
- The language is heavily influenced by Java and the JVM, taking inspiration from their design and concepts.
- Many features are yet to be implemented, and improvements will be made as development progresses.
- The language has not been extensively optimized for performance, but optimizations may be added in the future.

**Note**: This project was developed during my exploration of Rust, compilers, and virtual machines. Please note that
the code may not adhere to best practices. YuvaKriti is intended solely for educational purposes and is not recommended
for production use.

## Building YuvaKriti

To get started with YuvaKriti, you can clone the repository from GitHub:

```
git clone https://github.com/itsaky/yuvakriti.git
```

YuvaKriti is a Rust project, and you can build it using Cargo, the Rust package manager. Navigate to the project
directory and run:

```
cargo build
```

This will compile the project. To run the compiled binary, use:

```
cargo run -- <args>
```

Run with `--help` to see all configurations :

```
cargo run -- --help
```

## Hello World!

- Create a `hello.yk` source file :
  ```
  print "Hello World!";
  ```
  
- Compile the source file :
  ```
  cargo run -- compile hello.yk
  ```
  This will create a `hello.ykb` bytecode executable, which can be run using the YuvaKriti virtual machine.
  
- Run the program :
  ```
  cargo run -- run hello.ykb
  
  // Output :
  Hello World!
  ```

## License

```
Copyright (c) 2024 Akash Yadav
 
This program is free software: you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation, version 3.
 
This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 
You should have received a copy of the GNU General Public License along with this
program. If not, see <https://www.gnu.org/licenses/>.
```