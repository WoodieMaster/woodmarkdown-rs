# woodmarkdown-rs

Custom feature implementation on top of markdown-rs

## Links

* [GitHub: `WoodieMaster/woodmarkdown-rs`][repo]

## What is this?

An implementation of my own features for markdown, using the markdown-rs crate.


## Questions

* to learn markdown,
  see this [cheatsheet and tutorial][cheat]
* for the API,
  see the [crate docs][docs]
* for questions,
  see [Discussions][]
* to help,
  see [contribute][] below

## Contents

* [Install](#install)
* [Use](#use)
* [API](#api)
* [Extensions](#extensions)
* [Project](#project)
  * [Overview](#overview)
  * [File structure](#file-structure)
  * [Test](#test)
  * [Version](#version)
  * [Security](#security)
  * [Contribute](#contribute)
  * [Thanks](#thanks)
* [Related](#related)
* [License](#license)

## Test

`markdown-rs` is tested with the \~650 CommonMark tests and more than 1k extra
tests confirmed with CM reference parsers.
Then there’s even more tests for GFM and other extensions.
These tests reach all branches in the code,
which means that this project has 100% code coverage.
Fuzz testing is used to check for things that might fall through coverage.

`woodmarkdown-rs` has no additional tests yet.

The following bash scripts are useful when working on this project:

* generate code (latest CM tests and Unicode info):
  ```sh
  cargo run --manifest-path generate/Cargo.toml
  ```
* run examples:
  ```sh
  RUST_BACKTRACE=1 RUST_LOG=trace cargo run --example lib --features log
  ```
* format:
  ```sh
  cargo fmt && cargo fix --all-features --all-targets --workspace
  ```
* lint:
  ```sh
  cargo fmt --check && cargo clippy --all-features --all-targets --workspace
  ```
* test:
  ```sh
  RUST_BACKTRACE=1 cargo test --all-features --workspace
  ```
* docs:
  ```sh
  cargo doc --document-private-items --examples --workspace
  ```
* fuzz:
  ```sh
  cargo install cargo-fuzz
  cargo install honggfuzz
  cargo +nightly fuzz run markdown_libfuzz
  cargo hfuzz run markdown_honggfuzz
  ```

## Base Project

The base project markdown-rs can be found here:

* [GitHub: `wooorm/markdown-rs`][repo]
* [`crates.io`: `markdown`][crate]
* [`docs.rs`: `markdown`][docs]


## Contribute

See [`contributing.md`][contributing] for ways to help.
See [`code-of-conduct.md`][coc] for how to communicate in and around this
project.

## License

[MIT][license] © [Nico Holzmeister][author]

Original:

[MIT][license] © [Titus Wormer](https://github.com/wooorm)
