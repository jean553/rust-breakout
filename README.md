[![Build Status](https://travis-ci.org/jean553/rust-breakout.svg?branch=master)](https://travis-ci.org/jean553/rust-breakout)

# rust-breakout

A simple breakout video game developed using Rust.

## Compilation and development

Start the docker container.

```bash
vagrant up
```

Connect to the container.

```bash
vagrant ssh
```

Compile the program.

```bash
cargo build --release
```

## Run

On your host:

```bash
./rust-breakout/target/release/rust-breakout
```

## Generate documentation

```bash
cargo rustdoc -- --no-defaults
```
