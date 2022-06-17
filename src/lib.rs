use std::thread::JoinHandle;
use std::time::Instant;
use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

pub use crate::utils::spinner_names::SpinnerNames as Spinners;
use crate::utils::spinners_data::SPINNERS as SpinnersMap;

mod utils;

pub struct Spinner {
    sender: Sender<(Instant, Option<String>)>,
    join: Option<JoinHandle<()>>,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.join.is_some() {
            self.sender.send((Instant::now(), None)).unwrap();
            self.join.take().unwrap().join().unwrap();
        }
    }
}

impl Spinner {
    /// Create a new spinner along with a message
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    /// ```
    ///
    /// No Message:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new(Spinners::Dots, String::new());
    /// ```
    pub fn new(spinner: Spinners, message: String) -> Self {
        Self::new_inner(spinner, message, None)
    }

    /// Create a new spinner that logs the time since it was created
    pub fn with_timer(spinner: Spinners, message: String) -> Self {
        Self::new_inner(spinner, message, Some(Instant::now()))
    }

    fn new_inner(spinner: Spinners, message: String, start_time: Option<Instant>) -> Self {
        let spinner_name = spinner.to_string();
        let spinner_data = SpinnersMap
            .get(&spinner_name)
            .unwrap_or_else(|| panic!("No Spinner found with the given name: {}", spinner_name));

        let (sender, recv) = channel::<(Instant, Option<String>)>();

        let join = thread::spawn(move || 'outer: loop {
            let mut stdout = stdout();
            for frame in spinner_data.frames.iter() {
                let (do_stop, stop_time, stop_symbol) = match recv.try_recv() {
                    Ok((stop_time, stop_symbol)) => (true, Some(stop_time), stop_symbol),
                    Err(TryRecvError::Disconnected) => (true, None, None),
                    Err(TryRecvError::Empty) => (false, None, None),
                };

                let frame = stop_symbol.unwrap_or_else(|| frame.to_string());
                match start_time {
                    None => {
                        print!("\r{} {}", frame, message);
                    }
                    Some(start_time) => {
                        let now = stop_time.unwrap_or_else(Instant::now);
                        let duration = now.duration_since(start_time).as_secs_f64();
                        print!("\r{}{:>10.3} s\t{}", frame, duration, message);
                    }
                }

                stdout.flush().unwrap();

                if do_stop {
                    break 'outer;
                }

                thread::sleep(Duration::from_millis(spinner_data.interval as u64));
            }
        });

        Self {
            sender,
            join: Some(join),
        }
    }

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
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let mut sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop();
    /// ```
    pub fn stop(&mut self) {
        self.stop_inner(Instant::now(), None);
    }

    /// Stop with a symbol that replaces the spinner
    ///
    /// The symbol is a String rather than a Char to allow for more flexibility, such as using ANSI color codes.
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let mut sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop_with_symbol("🗸");
    /// ```
    ///
    /// ANSI colors (green checkmark):
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let mut sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop_with_symbol("\x1b[32m🗸\x1b[0m");
    /// ```
    pub fn stop_with_symbol(&mut self, symbol: &str) {
        self.stop_inner(Instant::now(), Some(symbol.to_owned()));
        println!();
    }

    /// Stops the spinner and prints a new line
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let mut sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop_with_newline();
    /// ```
    pub fn stop_with_newline(&mut self) {
        self.stop();
        println!();
    }

    /// Stops the spinner and prints the provided message
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let mut sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop_with_message("Finished loading things into memory!".into());
    /// ```
    pub fn stop_with_message(&mut self, msg: String) {
        self.stop();
        println!("\x1b[2K\r{}", msg);
    }

    /// Stops the spinner with a provided symbol and message
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let mut sp = Spinner::new(Spinners::Dots, "Loading things into memory...".into());
    ///
    /// sp.stop_and_persist("✔", "Finished loading things into memory!".into());
    /// ```
    pub fn stop_and_persist(&mut self, symbol: &str, msg: String) {
        self.stop();
        println!("\x1b[2K\r{} {}", symbol, msg);
    }

    fn stop_inner(&mut self, stop_time: Instant, stop_symbol: Option<String>) {
        self.sender
            .send((stop_time, stop_symbol))
            .expect("Could not stop spinner thread.");
        self.join.take().unwrap().join().unwrap();
    }
}
