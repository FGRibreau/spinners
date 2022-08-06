use std::thread::JoinHandle;
use std::time::Instant;
use std::{
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

pub use crate::utils::spinner_names::SpinnerNames as Spinners;
use crate::utils::spinners_data::SPINNERS as SpinnersMap;
pub use crate::utils::color::Color;
use crate::utils::color::colorize;
mod utils;

pub struct Spinner {
    sender: Sender<(Instant, Option<String>)>,
    join: Option<JoinHandle<()>>,
    stream: Stream
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
    #[must_use] 
    pub fn new(spinner: Spinners, message: String) -> Self  {
        Self::new_inner(spinner, message, None, None)
    }

     /// Create a new colored spinner along with a message
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners, Color};
    ///
    /// let sp = Spinner::new_with_color(Spinners::Dots, "Loading things into memory...".into(), Color::Blue);
    /// ```
    ///
    /// No Message:
    ///
    /// ```
    /// use spinners::{Spinner, Spinners};
    ///
    /// let sp = Spinner::new_with_color(Spinners::Dots, String::new(), None);
    /// ```
    pub fn new_with_color<T>(spinner: Spinners, message: String, color: T) -> Self 
    where T: Into<Option<Color>> + std::marker::Send + 'static + std::marker::Copy {
        Self::new_inner(spinner, message, color, None)
    }

    /// Create a new spinner that logs the time since it was created
    pub fn with_timer<T>(spinner: Spinners, message: String, color: T) -> Self 
    where T: Into<Option<Color>> + std::marker::Send + 'static + std::marker::Copy {
        Self::new_inner(spinner, message, color, Some(Instant::now()))
    }

    fn new_inner<T>(spinner: Spinners, message: String, color: T, start_time: Option<Instant>) -> Self 
    where T: Into<Option<Color>> + std::marker::Send + 'static + std::marker::Copy {
        let spinner_name = spinner.to_string();
        let spinner_data = SpinnersMap
            .get(&spinner_name)
            .unwrap_or_else(|| panic!("No Spinner found with the given name: {}", spinner_name));

        let stream = if let Some(stream) = stream { stream } else { Stream::default() };

        let (sender, recv) = channel::<(Instant, Option<String>)>();

        let join = thread::spawn(move || 'outer: loop {
            let mut stdout = stdout();
            for frame in &spinner_data.frames {
                let (do_stop, stop_time, stop_symbol) = match recv.try_recv() {
                    Ok((stop_time, stop_symbol)) => (true, Some(stop_time), stop_symbol),
                    Err(TryRecvError::Disconnected) => (true, None, None),
                    Err(TryRecvError::Empty) => (false, None, None),
                };

                let frame = stop_symbol.unwrap_or_else(|| (*frame).to_string());
                match start_time {
                    None => {
                        print!("\r{} {}", colorize(frame, color.into()), message);
                    }
                    Some(start_time) => {
                        let now = stop_time.unwrap_or_else(Instant::now);
                        let duration = now.duration_since(start_time).as_secs_f64();
                        print!("\r{}{:>10.3} s\t{}", colorize(frame, color.into()), duration, message);
                    }
                }

                stream.write(&frame, &message, start_time, stop_time).expect("IO Error");

                if do_stop {
                    break 'outer;
                }

                thread::sleep(Duration::from_millis(u64::from(spinner_data.interval)));
            }
        });

        Self {
            sender,
            join: Some(join),
            stream
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
        self.stream.stop(None, Some(symbol)).expect("IO error");
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
        self.stream.stop(None, None).expect("IO error");
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
        self.stream.stop(Some(&msg), None).expect("IO Error");
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
        self.stream.stop(Some(&msg), Some(symbol)).expect("IO Error");
    }

    fn stop_inner(&mut self, stop_time: Instant, stop_symbol: Option<String>) {
        self.sender
            .send((stop_time, stop_symbol))
            .expect("Could not stop spinner thread.");
        self.join.take().unwrap().join().unwrap();
    }
}
