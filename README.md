## tdd

> This app is written only for demonstration purposes.

Solution for the [Exercise 3 - To-do list client application](https://github.com/timotr/harjutused/blob/main/hajusrakendused/todo-client.md) written in Rust. 

## Usage

### Cloning the project and installing the dependencies: 

- [clap](https://crates.io/crates/clap)
- [reqwest](https://crates.io/crates/reqwest)
- [tokio](https://crates.io/crates/tokio)
- [colored](https://crates.io/crates/colored)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)

```
git clone https://github.com/ivanivanovich1/tdd
cd tdd
cargo run
```
This will launch the program in a development environment. 

In order to build a binary file, command
```
cargo build --release
```
could be used.

For further information use:
```
cargo r -- -h
```
