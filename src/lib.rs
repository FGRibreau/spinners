use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

pub mod utils;
pub use crate::utils::spinner_names::SpinnerNames as Spinners;
use crate::utils::spinners_data::SPINNERS as SpinnersMap;

pub struct Spinner {
    sender: Sender<()>,
}

impl Spinner {
    /// Create a new spinner along with a message
    ///
    /// If you desire an empty message, create an empty string
    ///
    /// # Basic Usage:
    ///
    /// ```
    /// use spinners_rs::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    /// ```
    pub fn new(spinner: Spinners, message: String) -> Self {
        let spinner_name = spinner.to_string();
        let spinner_data = SpinnersMap
            .get(&spinner_name)
            .unwrap_or_else(|| panic!("No Spinner found with the given name: {}", spinner_name));

        let (sender, recv) = channel::<()>();

        thread::spawn(move || 'outer: loop {
            let mut stdout = stdout();
            for frame in spinner_data.frames.iter() {
                match recv.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }

                print!("\r{} {}", frame, message);
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(spinner_data.interval as u64));
            }
        });

        Spinner { sender }
    }

    // TODO: Add update message function

    /// Stops the spinner
    ///
    /// Stops the spinner that was created with the [`Spinner::new`] function.
    ///
    /// Optionally call [`stop_with_newline`] to print a newline after the spinner is stopped,
    /// or the [`stop_with_message`] function to print a message after the spinner is stopped.
    ///
    /// [`Spinner::new`]: struct.Spinner.html#method.new
    /// [`stop_with_newline`]: struct.Spinner.html#method.stop_with_newline
    /// [`stop_with_message`]: struct.Spinner.html#method.stop_with_message
    ///
    /// # Basic Usage:
    ///
    /// ```
    /// use spinners_rs::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop();
    /// ```
    pub fn stop(self) {
        self.sender
            .send(())
            .expect("Could not stop spinner thread.");
    }

    /// Stops the spinner and prints a new line
    ///
    /// # Basic Usage:
    ///
    /// ```
    /// use spinners_rs::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop_with_newline();
    /// ```
    pub fn stop_with_newline(self) {
        self.stop();
        println!();
    }

    /// Stops the spinner and prints the provided message
    ///
    /// # Basic Usage:
    ///
    /// ```
    /// use spinners_rs::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop_with_message("Finished loading things into memory!".into());
    /// ```
    pub fn stop_with_message(self, msg: String) {
        self.stop();
        print!("\r{}", msg);
    }
}
