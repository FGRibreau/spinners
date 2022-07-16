use spinners::{Spinner, Spinners, StopBehavior};

fn frobnicate(should_work: bool) -> Result<(), String> {
    if should_work {
        Ok(())
    } else {
        Err("Error!".to_string())
    }
}

fn main() -> Result<(), String> {
    // Successful variant shows the message from [Spinner::stop_with]
    // and not the one from [Spinner::on_drop]
    {
        println!("Spinner that is stopped after some action is successfully performed:");
        let mut sp = Spinner::new(Spinners::Dots, "Frobincating...".into()).on_drop(
            StopBehavior::SymbolAndMessage(
                "✗",
                "Ouch, something went wrong with the Frobnicator!?".into(),
            ),
        );
        frobnicate(true)?;
        sp.stop_with(&StopBehavior::SymbolAndMessage(
            "✔",
            "Frobnication successful!".into(),
        ));
    }

    // Failure variant in which the Spinner is dropped before having been explicitly
    // stopped, the behavior passed into [Spinner::on_drop] is used.
    {
        println!("Spinner that never reaches a stop call because an error is hit before:");
        let mut sp = Spinner::new(Spinners::Dots, "Frobincating...".into()).on_drop(
            StopBehavior::SymbolAndMessage(
                "✗",
                "Ouch, something went wrong with the Frobnicator!?".into(),
            ),
        );
        frobnicate(false)?;
        sp.stop_with(&StopBehavior::SymbolAndMessage(
            "✔",
            "Frobnication successful!".into(),
        ));
    }
    Ok(())
}
