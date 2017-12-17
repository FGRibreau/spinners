extern crate spinner;
#[macro_use]
extern crate lazy_static;

extern crate strum;
#[macro_use]
extern crate strum_macros;

mod spinner_data;
mod spinners_data;
mod spinner_names;

use std::time::Duration;

pub use spinner_names::SpinnerNames as Spinners;
pub use spinners_data::SPINNERS as RawSpinners;

pub struct Spinner {
    handle: spinner::SpinnerHandle,
}

impl Spinner {
    /// Create a new spinner along with a message
    ///
    /// Returns a spinner
    pub fn new(spinner: Spinners, message: String) -> Self {
        let spinner_name = format!("{:?}", spinner);
        let spinner_data = RawSpinners
            .iter()
            .find(|x| x.name == spinner_name)
            .take()
            .unwrap()
            .clone();

        // @todo implement my own Spinner thread
        let handle = spinner::SpinnerBuilder::new(message)
            .spinner(spinner_data.frames.clone())
            .step(Duration::from_millis(spinner_data.interval.into()))
            .start();

        Spinner { handle: handle }
    }

    /// Update spinner's message
    ///
    /// Returns the String that is put in in case the sender could not send.
    pub fn message(&self, message: String) -> Option<String> {
        self.handle.update(message)
    }

    /// Stop the spinner
    pub fn stop(self) {
        self.handle.close();
    }
}
