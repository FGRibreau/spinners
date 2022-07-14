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

struct SpinnerHandle {
    sender: Sender<(Instant, Option<String>)>,
    join: JoinHandle<()>,
}

pub struct Spinner {
    handle: Option<SpinnerHandle>,
    on_drop: Option<StopBehavior>,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.handle.is_some() {
            let behavior = self.on_drop.take().unwrap();
            self.stop_with(&behavior);
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
            handle: Some(SpinnerHandle { sender, join }),
            on_drop: Some(StopBehavior::NewLine),
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
        let hdl = self.handle.take().unwrap();

        hdl.sender
            .send((stop_time, stop_symbol))
            .expect("Could not stop spinner thread.");

        hdl.join.join().unwrap();
    }

    /// Builder method  to convert a Spinner to a one with a new [StopBehavior].
    ///
    /// If the spinner is dropped without any prior call to one of its `Spinner::stop*`
    /// methods, then the [StopBehavior] from the `behavior` parameter is triggered.
    ///
    /// Example:
    /// ```
    /// use spinners::{Spinner, Spinners, StopBehavior};
    ///
    /// # fn foo() -> Result<(), String> {
    /// fn frobnicate(should_work: bool) -> Result<(), String> {
    ///     if should_work {
    ///         Ok(())
    ///     } else {
    ///         Err("Error!".to_string())
    ///     }
    /// }
    ///
    /// // Successful variant shows the message from [Spinner::stop_with]
    /// // and not the one from [Spinner::on_drop]
    /// {
    ///     let mut sp = Spinner::new(Spinners::Dots, "Frobincating...".into())
    ///         .on_drop(StopBehavior::SymbolAndMessage(
    ///             "✗",
    ///             "Ouch, something went wrong with the Frobnicator!?".into()));
    ///     frobnicate(true)?;
    ///     sp.stop_with(&StopBehavior::SymbolAndMessage(
    ///         "✔", "Frobnication successful!".into()));
    /// }
    ///
    /// // Failure variant in which the Spinner is dropped before having been explicitly
    /// // stopped, the behavior passed into [Spinner::on_drop] is used.
    /// {
    ///     let mut sp = Spinner::new(Spinners::Dots, "Frobincating...".into())
    ///         .on_drop(StopBehavior::SymbolAndMessage(
    ///             "✗",
    ///             "Ouch, something went wrong with the Frobnicator!?".into()));
    ///     frobnicate(false)?;
    ///     sp.stop_with(&StopBehavior::SymbolAndMessage(
    ///         "✔", "Frobnication successful!".into()));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn on_drop(mut self, behavior: StopBehavior) -> Self {
        let hdl = self.handle.take().unwrap();
        Self {
            handle: Some(SpinnerHandle {
                sender: hdl.sender,
                join: hdl.join,
            }),
            on_drop: Some(behavior),
        }
    }

    /// Stops the spinner with a given `StopBehavior`
    ///
    /// Each StopBehavior matches one of the other `Spinner::stop*` methods.
    pub fn stop_with(&mut self, stop_behavior: &StopBehavior) {
        match stop_behavior {
            StopBehavior::NewLine => self.stop_with_newline(),
            StopBehavior::Message(msg) => self.stop_with_message(msg.to_owned()),
            StopBehavior::Symbol(symbol) => self.stop_with_symbol(symbol),
            StopBehavior::SymbolAndMessage(symbol, msg) => {
                self.stop_and_persist(symbol, msg.to_owned())
            }
        }
    }
}

pub enum StopBehavior {
    NewLine,
    Message(String),
    Symbol(&'static str),
    SymbolAndMessage(&'static str, String),
}
