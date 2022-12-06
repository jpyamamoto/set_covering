# Set Covering

Approximation Algorithm for the Set Covering problem.

## Run Locally

The project was written in Rust 1.63. Make sure to have the `rustc` compiler and `cargo` package manager installed.

```bash
rustc --version
cargo --version
```

Clone the project

```bash
git clone https://github.com/jpyamamoto/set_covering
```

Go to the project directory

```bash
cd set_covering
```

Build the project

```bash
cargo build --release
```

### Install

If you want the binary to be locally available, make sure the directory `$HOME/.cargo/bin` is in your `$PATH` and run the following command

```bash
cargo install --path .
```

Now the binary is locally available in your PATH

```bash
set_covering
```

### Run Without Installing

If you want to run the project without installing install

```bash
cargo run
```

## Authors

- [@jpyamamoto](https://www.github.com/jpyamamoto)

