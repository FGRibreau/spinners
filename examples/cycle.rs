use spinners::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};
use strum::IntoEnumIterator;

fn main() {
    // loop through each spinner and display them for 2 seconds
    for spinner in Spinners::iter() {
        let mut sp = Spinner::new(spinner.clone(), format!("{:?}", spinner));
        sleep(Duration::from_secs(2));
        sp.stop();
    }
}
