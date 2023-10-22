# Rust

## Install

Install Rust toolchain
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

>Note:
>Rustup is the Rust toolchain manager and build manager


Command line commands
```bash
rustup --version
cargo install mdbook # Package install
rustc file.rs # Compile file to executable; usually don't use this command directly.
```


### Cargo
Install cargo generate
`cargo install cargo-generate`

```bash
cargo new hello # Creates new project in folder called hello
cargo run # Compiles a project into /target folder and runs /target
cargo build # Compiles a project into /target folder optimized for development
cargo build --release # Compiles a project into /target folder optimized for production
```
