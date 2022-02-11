use spinners_rs::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new(&Spinners::Dots9, "Waiting for 3 seconds".into());
    sleep(Duration::from_secs(3));
    sp.stop();
}
