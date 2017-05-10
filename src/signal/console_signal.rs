use signal::{Signal, SignalError};
use api_enquirer::StatusReport;

pub struct ConsoleSignal;

impl Signal for ConsoleSignal {
    fn signal_for_pin(&self, status_report: &StatusReport) -> Result<(), SignalError> {
        println!("For pin {}: {}", status_report.pin_id, status_report.status);

        Ok(())
    }
}