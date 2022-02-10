use std::{
    io::{self, Write},
    sync::mpsc::{self, Sender, TryRecvError},
    thread,
    time::Duration,
};

pub mod utils;
pub use crate::utils::spinner_names::SpinnerNames as Spinners;
use crate::utils::spinners_data::SPINNERS as RawSpinners;

pub struct Spinner {
    sender: Sender<()>,
}

impl Spinner {
    /// Create a new spinner along with a message
    pub fn new(spinner: &Spinners, message: String) -> Self {
        let spinner_name = format!("{:?}", spinner);
        let spinner_data = match RawSpinners.get(&spinner_name) {
            Some(spinner_data) => spinner_data,
            None => panic!("No Spinner found with the given name: {}", spinner_name),
        };

        let (tx, rx) = mpsc::channel::<()>();

        thread::spawn(move || loop {
            for frame in spinner_data.frames.iter() {
                print!("\r{} {}", message, frame);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(spinner_data.interval as u64));
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Terminating.");
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        });

        Spinner { sender: tx }
    }

    // TODO: Add update message function

    /// Stop the spinner
    pub fn stop(self) {
        self.sender
            .send(())
            .expect("Could not stop spinner thread.");
    }
}
