# The cryptopals crypto challenges

This repository contains my solutions to the [cryptopals crypto challenges](https://cryptopals.com/), implemented in
Rust.

The goal is to have the minimum amount of dependencies, writing as much custom implementations as possible. Therefore,
the [`utils`](./src/utils) crate contains many custom implementations, including Base64 encoding and decoding, as well
as AES-128 in ECB and CBC mode.

## Usage

This project uses Rust, so be sure to have `rustup` and `cargo` installed before continuing
(see [https://rust-lang.org/tools/install/](https://rust-lang.org/tools/install/)). To build and run the project, use
`cargo run --release`.

Each set of the challenges is implemented in a crate named `setN`, where `N` is the set number. Each set crate contains
one module per challenge. For instance, the challenge 2 of set 1 is implemented in [
`src/set1/chall2.rs`](./src/set1/chall2.rs).

The binary entry point is in [`src/main.rs`](./src/main.rs), and is used to select the challenge (s) to run.

Each challenge uses assertions to ensure the implementations give the expected result. This means that the expected
behavior of the program is to run and exit with code `0`.

## Unit tests

The `utils` crate contains some unit tests, to check the behavior of the custom implementations. These tests can be run
with `cargo test`.