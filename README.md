# Advent of code 2023

## Install

This project uses `nix` flake to set up dependencies. If you're on Apple silicon, you can run the following to install the toolchain. There should be a similar one for other architectures.

```console
$ rustup toolchain install stable-aarch64-apple-darwin
```

## Run solutions

Run the following where `{day}` is `day1`, `day2`, ...

```console
$ cargo run {day}
```

Tests are also available as I've used them to build each of the solutions

```console
$ cargo test {day}
```
