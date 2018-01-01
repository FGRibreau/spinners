# Spinners - 🛎 60+ Elegant terminal spinners for Rust

<!--[![Travis](https://img.shields.io/travis/rust-lang/rust.svg)](https://travis-ci.org/FGRibreau/spinners) [![codecov](https://codecov.io/gh/FGRibreau/spinners/branch/master/graph/badge.svg)](https://codecov.io/gh/FGRibreau/spinners) -->
[![Cargo version](https://img.shields.io/crates/v/spinners.svg)](https://crates.io/crates/spinners) [![Crates.io](https://img.shields.io/crates/l/spinners.svg)](https://crates.io/crates/spinners) [![docs.rs](https://img.shields.io/badge/docs.rs-👌-4EC329.svg?)](https://docs.rs/spinners/) [![Crates.io](https://img.shields.io/crates/d/spinners.svg)](https://crates.io/crates/spinners)

<p align="center"><img src="https://media.giphy.com/media/3oxHQyZfOJjlL3bhRK/giphy.gif"></p>


## ❤️ Shameless plug

- [**Charts, simple as a URL**. No more server-side rendering pain, 1 url = 1 chart](https://image-charts.com)
- [Looking for a free **Redis GUI**?](http://redsmin.com) [Or for **real-time alerting** & monitoring for Redis?](http://redsmin.com)

## Install

See [Cargo page](https://crates.io/crates/spinners)

## Usage

```rust
extern crate spinners;

use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sp = Spinner::new(Spinners::Dots9, "Waiting for 3 seconds".into());
    sleep(Duration::from_secs(3));
    sp.stop();
}
```

- [List of available spinners](src/spinner_names.rs)
- [Documentation](https://docs.rs/spinners/)

## Example

```shell
cargo run --example cycle
```

```shell
cargo run --example simple
```


## License

MIT © [François-Guillaume Ribreau](https://fgribreau.com)
