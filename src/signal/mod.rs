pub mod console_signal;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use api_enquirer::StatusReport;

pub trait Signal {
    fn signal_for_pin(&self, status_report: &StatusReport) -> Result<(), SignalError>;
}

#[derive(Debug)]
pub struct SignalError {
    description: String
}

impl Error for SignalError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Display for SignalError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}