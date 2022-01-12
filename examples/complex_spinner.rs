use spinners::{Spinner, Spinners};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sp = Spinner::extended(
        &Spinners::Pipe,
        "Waiting for 3 seconds".into(),
        Box::new(|sp, status| {
            format!(
                "{spin} -- Currently working on: \'{status}\' -- {spin}",
                spin = sp,
                status = status
            )
        }),
    );
    sleep(Duration::from_secs(3));
    sp.stop();
}
