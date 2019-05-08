# Setup
## Installation
- Install Rust through `rustup`:  
`$ curl https://sh.rustup.rs -sSf | sh`
- Add Rust to system PATH:  
`$ source $HOME/.cargo/env`
- Check if Rust is installed correctly:  
`$ rustc --version`
- Create `main.rs`:  
`$ touch main.rs`
- Enter the following code into `main.rs`:
    ```
    fn main() {
        println!("Hello, world!");
    }
    ```
- Compile `main.rs`:   
`$ rustc main.rs`
- Run `main.rs`:  
`$ ./main`

## Dependencies
Manage dependencies with `Cargo`, Rust's build system and package manager.
- Check if Cargo is installed:  
`$ cargo --version`
- Setup Cargo project:  
`$ cargo new hello_cargo`  
`$ cd hello_cargo`  
`$ ls -al`
- We have a `Cargo.toml` file which describes our project and it's dependencies
- Individual dependencies are referred to as *crates*. The dependencies section is empty for now. We will add some dependencies later.
- Cargo also created a `src` folder for us and generated a `main.rs` file with a simple Hello, world! program already written.
- Now, we can use `cargo build` to build our program instead of manually compiling with `rustc`:
- Build the program:  
`$ cargo build`
- You should now have a `Cargo.lock` file and the compiled program will be at `<root>/target/debug/hello_cargo`
- Run the program
`./target/debug/hello_cargo`
- Use `cargo run` to build and run the program in one command:  
`$ cargo run`
- Use `cargo check` to check if your code compiles, without producing an executable  
`$ cargo check`
- When the project is ready to release, use the following:  
`cargo build --release`
  - Building with the `--release` flag will take longer to compile but will perform optimizations to make your code run faster. You don't need the optimizations while in development, so you should opt for faster compile times. If we build this project with the `--release` flag, the resulting executable will be located at `<root>/target/release/hello_cargo`