use spinners::{Color, Spinner, Spinners};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut sp = Spinner::new_with_color(
        Spinners::Dots9,
        "Waiting for 3 seconds".into(),
        Color::Green,
    );
    sleep(Duration::from_secs(3));
    sp.stop_with_message("Finishing waiting for 3 seconds".into());
}
