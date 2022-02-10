# Spinners - 🛎 60+ Elegant terminal spinners for Rust

[![Cargo version](https://img.shields.io/crates/v/spinners-rs.svg)](https://crates.io/crates/spinners-rs) [![Crates.io](https://img.shields.io/crates/l/spinners-rs.svg)](https://crates.io/crates/spinners-rs) [![docs.rs](https://img.shields.io/badge/docs.rs-👌-4EC329.svg?)](https://docs.rs/spinners-rs/) [![Crates.io](https://img.shields.io/crates/d/spinners-rs.svg)](https://crates.io/crates/spinners-rs) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)

<p align="center"><img src="https://media.giphy.com/media/3oxHQyZfOJjlL3bhRK/giphy.gif"></p>

## Install

See [Cargo page](https://crates.io/crates/spinners-rs)

## Usage

```rust
use spinners;

use spinners::{Spinner, Spinners};
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
