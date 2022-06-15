use spinners::{Spinner, Spinners};
use std::{env, str::FromStr, thread::sleep, time::Duration};

fn main() {
    let mut args = env::args();
    let spinner_name = args.nth(1).unwrap_or_else(|| "Dots9".to_string());

    let mut sp = Spinner::new(
        Spinners::from_str(&spinner_name).unwrap(),
        "Waiting for 3 seconds".into(),
    );
    sleep(Duration::from_secs(3));
    sp.warn("This is a warning".to_string())
}
