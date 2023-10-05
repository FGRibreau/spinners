# Spinners - 🛎 60+ Elegant terminal spinners for Rust

[![Cargo version](https://img.shields.io/crates/v/spinners.svg)](https://crates.io/crates/spinners) [![Crates.io](https://img.shields.io/crates/l/spinners.svg)](https://crates.io/crates/spinners) [![docs.rs](https://img.shields.io/badge/docs.rs-👌-4EC329.svg?)](https://docs.rs/spinners/) [![Crates.io](https://img.shields.io/crates/d/spinners.svg)](https://crates.io/crates/spinners) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)

<p align="center"><img src="https://media.giphy.com/media/3oxHQyZfOJjlL3bhRK/giphy.gif"></p>

> ## ❤️ Shameless plug
> - [Open-Source **Webhook** as a Service](https://www.hook0.com/)
> - [**Charts, simple as a URL**. 1 url = 1 chart - Charts API](https://image-charts.com)
> - [Keycloak Identity and Access Management (IAM) as a Service](https://www.cloud-iam.com/)
> - [Automate your **RoamResearch** second brain](https://www.roam-bot.com)
> - [Blazing Fast Gitlab CI Runners (10x faster)](https://cloud-runner.com/)


![200083093-cf48fcab-d95c-4a59-ac66-6e167dd33e7e](https://github.com/FGRibreau/spinners/assets/138050/a3e4d4f9-44c4-4b54-82a7-e608ab1da742)

## Install

See [Cargo page](https://crates.io/crates/spinners)

## Usage

```rust
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut sp = Spinner::new(Spinners::Dots9, "Waiting for 3 seconds".into(), None);
    sleep(Duration::from_secs(3));
    sp.stop();
}
```

- [List of available spinners](src/utils/spinner_names.rs)
- [Documentation](https://docs.rs/spinners/)

## Example

```shell
cargo run --example cycle
```

```shell
cargo run --example simple
```

## License

MIT © [François-Guillaume Ribreau](https://fgribreau.com), James Cordor
