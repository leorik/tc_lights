#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate hyper;

use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};

use api_enquirer::StatusReport;

mod api_enquirer;
mod settings;
mod threads;
mod signal;

fn main() {
    let settings = settings::read_settings().unwrap();

    let (sender, receiver) : (Sender<Vec<StatusReport>>, Receiver<Vec<StatusReport>>) = channel();
    let settings_arc = Arc::new(settings);
    let enquirer_handle = threads::run_enquirer_thread(settings_arc.clone(), sender);
    let signal_handle = threads::run_signal_thread(settings_arc.clone(), receiver);

    signal_handle.join().unwrap();
    enquirer_handle.join().unwrap();
}
