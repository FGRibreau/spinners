use spinners_rs::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};
use strum::IntoEnumIterator;

fn main() {
    // loop through each spinner and display them forc 2 seconds
    for spinner in Spinners::iter() {
        let sp = Spinner::new(spinner.clone(), format!("{:?}", spinner));
        sleep(Duration::from_secs(2));
        sp.stop();
    }
}
