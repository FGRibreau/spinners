use spinner::{SpinnerBuilder, SpinnerHandle};
use std::time::Duration;

pub mod utils;
pub use crate::utils::spinner_names::SpinnerNames as Spinners;
use crate::utils::spinners_data::SPINNERS as RawSpinners;

type FormatFn = dyn Fn(&str, &str) -> String + Send + 'static;

pub struct Spinner {
    handle: SpinnerHandle,
}

impl Spinner {
    /// Create a new spinner along with a message
    ///
    /// Returns a spinner
    pub fn new(spinner: &Spinners, message: String) -> Self {
        let spinner_name = format!("{:?}", spinner);
        let spinner_data = match RawSpinners.get(&spinner_name) {
            Some(spinner_data) => spinner_data,
            None => panic!("No Spinner found with the given name: {}", spinner_name),
        };

        // @todo implement my own Spinner thread
        let handle = SpinnerBuilder::new(message)
            .spinner(spinner_data.frames.clone())
            .step(Duration::from_millis(spinner_data.interval.into()))
            .start();

        Spinner { handle }
    }

    /// Create a new spinner along with a message
    ///
    /// Returns a spinner
    pub fn extended(spinner: &Spinners, ex: Box<FormatFn>) -> Self {
        let spinner_name = format!("{:?}", spinner);
        let spinner_data = match RawSpinners.get(&spinner_name) {
            Some(spinner_data) => spinner_data,
            None => panic!("No Spinner found with the given name: {}", spinner_name),
        };

        // @todo implement my own Spinner thread
        let handle = SpinnerBuilder::new("".into())
            .format(ex)
            .spinner(spinner_data.frames.clone())
            .step(Duration::from_millis(spinner_data.interval.into()))
            .start();

        Spinner { handle }
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
