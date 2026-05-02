# Spinners - 🛎 60+ Elegant terminal spinners for Rust

[![Cargo version](https://img.shields.io/crates/v/spinners.svg)](https://crates.io/crates/spinners) [![Crates.io](https://img.shields.io/crates/l/spinners.svg)](https://crates.io/crates/spinners) [![docs.rs](https://img.shields.io/badge/docs.rs-👌-4EC329.svg?)](https://docs.rs/spinners/) [![Crates.io](https://img.shields.io/crates/d/spinners.svg)](https://crates.io/crates/spinners) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)

<p align="center"><img src="https://media.giphy.com/media/3oxHQyZfOJjlL3bhRK/giphy.gif"></p>

## Sponsors

<table>
  <tr>
    <td align="center" width="175">
      <a href="https://france-nuage.fr/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=france-nuage&mtm_content=spinners">
        <img src="assets/sponsors/france-nuage.svg" height="60" alt="France-Nuage"/><br/>
        <b>France-Nuage</b>
      </a><br/>
      <sub>Sovereign French cloud for the CLIs you ship. Re-internalisable infra.</sub>
    </td>
    <td align="center" width="175">
      <a href="https://www.hook0.com/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=hook0&mtm_content=spinners">
        <img src="assets/sponsors/hook0.png" height="60" alt="Hook0"/><br/>
        <b>Hook0</b>
      </a><br/>
      <sub>Add webhook delivery to your CLI's backend. Self-hosted, retries done.</sub>
    </td>
    <td align="center" width="175">
      <a href="https://getnatalia.com/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=natalia&mtm_content=spinners">
        <img src="assets/sponsors/natalia.svg" height="60" alt="Natalia"/><br/>
        <b>Natalia</b>
      </a><br/>
      <sub>Spinning while users wait? Natalia answers their calls 24/7 meanwhile.</sub>
    </td>
    <td align="center" width="175">
      <a href="https://netir.fr/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=netir&mtm_content=spinners">
        <img src="assets/sponsors/netir.svg" height="60" alt="NetIR"/><br/>
        <b>NetIR</b>
      </a><br/>
      <sub>Hire vetted French freelance CLI authors or join the marketplace.</sub>
    </td>
  </tr>
  <tr>
    <td align="center" width="233">
      <a href="https://nobullshitconseil.com/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=nbc&mtm_content=spinners">
        <img src="assets/sponsors/nobullshitconseil.svg" height="60" alt="NoBullshitConseil"/><br/>
        <b>NoBullshitConseil</b>
      </a><br/>
      <sub>Tech advisory without the bullshit. Straight talk for founders.</sub>
    </td>
    <td align="center" width="233">
      <a href="https://qualneo.fr/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=qualneo&mtm_content=spinners">
        <img src="assets/sponsors/qualneo.svg" height="60" alt="Qualneo"/><br/>
        <b>Qualneo</b>
      </a><br/>
      <sub>Qualiopi LMS for French trainers — friendly nod from a CLI builder.</sub>
    </td>
    <td align="center" width="233">
      <a href="https://recapro.ai/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=recapro&mtm_content=spinners">
        <img src="assets/sponsors/recapro.png" height="60" alt="Recapro"/><br/>
        <b>Recapro</b>
      </a><br/>
      <sub>Sovereign AI for meeting notes. Less spinner, more shipped reports.</sub>
    </td>
  </tr>
</table>

> **Interested in sponsoring?** [Get in touch](mailto:rust@fgribreau.com)


![200083093-cf48fcab-d95c-4a59-ac66-6e167dd33e7e](https://github.com/FGRibreau/spinners/assets/138050/a3e4d4f9-44c4-4b54-82a7-e608ab1da742)

## Install

See [Cargo page](https://crates.io/crates/spinners)

## Usage

```rust
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut sp = Spinner::new(Spinners::Dots9, "Waiting for 3 seconds".into());
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

## Feature flags

### `osc-progress`

Enables native terminal progress bar support via the [ConEmu OSC 9;4](https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC) protocol. Terminals like Ghostty, Windows Terminal, iTerm2, Kitty, and WezTerm render these as GUI progress bars in the title/tab bar. Unsupported terminals silently ignore the sequences.

```toml
[dependencies]
spinners = { version = "4.1.0", features = ["osc-progress"] }
```

The progress bar is emitted as an indeterminate/pulsing indicator while the spinner is active, and cleared when the spinner is stopped or dropped. Sequences are only emitted when the output stream is a terminal, so piped output is unaffected.

**Signal handling caveat:** If the process is killed abruptly (e.g. `SIGINT` via Ctrl+C, `SIGKILL`), the `Drop` implementation may not run and the progress bar won't be cleared. Terminals like Ghostty mitigate this with a ~15-second auto-clear timeout, but for immediate cleanup, applications should install their own signal handler that stops the spinner (e.g. by dropping it or calling `.stop()`) before exiting.

## License

MIT © [François-Guillaume Ribreau](https://fgribreau.com), James Cordor
