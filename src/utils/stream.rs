use std::{io::{Write, stdout, stderr, Result}, time::Instant};

/// Handles the Printing logic for the Spinner
#[derive(Default, Copy, Clone)]
pub enum Stream {
    #[default]
    Stderr,
    Stdout,
}
impl Stream {
    /// Matches on self and returns the internal writer
    fn match_target(&self) -> Box<dyn Write> {
        match self {
            Self::Stderr => Box::new(stderr()),
            Self::Stdout => Box::new(stdout())
        }
    }

    /// Writes the message without duration
    fn print_message(
            writer: &mut Box<dyn Write>, 
            frame: &str, 
            message: &str) -> Result<()>
    {
        write!(writer, "\r{} {}", frame, message)?;
        writer.flush()
    }

    /// Writes the message with the duration
    fn print_message_with_duration(
            writer: &mut Box<dyn Write>, 
            frame: &str, 
            message: &str, 
            start_time: Instant,
            stop_time: Option<Instant>) -> Result<()> 
    {
        let now = stop_time.unwrap_or_else(Instant::now);
        let duration = now.duration_since(start_time).as_secs_f64();
        write!(writer, "\r{}{:>10.3} s\t{}", frame, duration, message)?;
        writer.flush()
    }

    /// Writes the current message and optionally prints the durations
    pub fn write(
            &self,
            frame: &str, 
            message: &str, 
            start_time: Option<Instant>,
            stop_time: Option<Instant>) -> Result<()>
    {
        let mut writer = self.match_target();
        match start_time {
            None => Self::print_message(
                &mut writer, frame, message)?,
            Some(start_time) => Self::print_message_with_duration(
                &mut writer, frame, message, start_time, stop_time)?
        };
        Ok(())
    }

    /// Handles the stopping logic given an optional message and symbol
    pub fn stop(
            &self,
            message: Option<&str>,
            symbol: Option<&str>) -> Result<()> 
    {
        let mut writer = self.match_target();
        match (message, symbol) {
            // persist with symbol and message
            (Some(m), Some(s)) => writeln!(writer, "\x1b[2K\r{} {}", s, m),

            // persist with message only
            (Some(m), None) => writeln!(writer, "\x1b[2K\r{}", m),

            // simple newline
            _ => writeln!(writer)
        }?;
        writer.flush()
    }
}
