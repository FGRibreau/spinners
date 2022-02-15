use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

pub mod utils;
pub use crate::utils::spinner_names::SpinnerNames as Spinners;
pub use crate::utils::spinners_data::SPINNERS as SpinnersMap;

pub struct Spinner {
    sender: Sender<()>,
}

impl Spinner {
    /// Create a new spinner along with a message
    pub fn new(spinner: Spinners, message: String) -> Self {
        let spinner_name = spinner.to_string();
        let spinner_data = SpinnersMap
            .get(&spinner_name)
            .expect(format!("No Spinner found with the given name: {}", spinner_name).as_str());

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

    /// Stop the spinner
    pub fn stop(self) {
        self.sender
            .send(())
            .expect("Could not stop spinner thread.");
    }

    /// Stop the spinner and print a new line
    pub fn stop_with_newline(self) {
        self.stop();
        print!("\n");
    }
}
