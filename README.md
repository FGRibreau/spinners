# Spinners RS - 🛎 60+ Elegant terminal spinners for Rust

A fork of [Spinners](https://github.com/FGRibreau/spinners) with improved performance and support for all modern terminals

[![Cargo version](https://img.shields.io/crates/v/spinners-rs.svg)](https://crates.io/crates/spinners-rs)
[![License](https://img.shields.io/github/license/jamesinaxx/spinners-rs)](https://github.com/jamesinaxx/spinners-rs/blob/master/LICENSE)
[![Docs](https://img.shields.io/badge/docs-👌-4EC329.svg?)](https://docs.rs/spinners-rs/)
[![Downloads](https://img.shields.io/crates/d/spinners-rs.svg)](https://crates.io/crates/spinners-rs)

![Demo Gif](https://media.giphy.com/media/3oxHQyZfOJjlL3bhRK/giphy.gif)

## Install

See [Cargo page](https://crates.io/crates/spinners-rs)

## Usage

```rust
use spinners_rs::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sp = Spinner::new(Spinners::Dots9, "Waiting for 3 seconds".into());
    sleep(Duration::from_secs(3));
    sp.stop();
}
```

- [List of available spinners](src/utils/spinner_names.rs)
- [Documentation](https://docs.rs/spinners-rs/)

## Example

```shell
cargo run --example cycle
```

```shell
cargo run --example simple
```

## License

MIT © James Cordor
