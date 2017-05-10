use signal::{Signal, SignalError};
use api_enquirer::BuildStatus;

pub struct ConsoleSignal;

impl Signal for ConsoleSignal {
    fn signal_for_pin(pin_id: &String, status: &BuildStatus) -> Result<(), SignalError> {
        println!("For pin {}: {}", pin_id, status);

        Ok(())
    }
}