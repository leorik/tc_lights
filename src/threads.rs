use std::thread::{spawn, sleep, JoinHandle};
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};

use api_enquirer::{Enquirer, StatusReport, BuildStatus};
use settings::LightsSettings;
use signal::*;
use signal::console_signal::ConsoleSignal;

const ENQUIRER_SLEEP_TIME_IN_SECS: u64 = 30;
const SIGNAL_SLEEP_TIME_IN_SECS: u64 = 1;

pub fn run_enquirer_thread(settings: Arc<LightsSettings>, status_channel : Sender<Vec<StatusReport>>) -> JoinHandle<()> {
    let enquirer = Enquirer::new();
    let duration = Duration::from_secs(ENQUIRER_SLEEP_TIME_IN_SECS);

    spawn(move || {
        loop {
            let local_settings = settings.as_ref();

            let mut statuses : Vec<StatusReport> = Vec::new(); // TODO change to array
            for project in local_settings.projects.iter() {
                statuses.push(
                    enquirer.query_for_project(&local_settings.teamcity_url, project).unwrap_or_else(|err| {
                        println!("Error during status query on project {} : {}", &project.pin_id, err);

                        StatusReport { pin_id: project.pin_id.clone(), status: BuildStatus::Unknown }
                    }));
            }

            status_channel.send(statuses).unwrap();

            sleep(duration);
        }
    })
}

pub fn run_signal_thread(settings: Arc<LightsSettings>, status_channel : Receiver<Vec<StatusReport>>) -> JoinHandle<()> {
    let duration = Duration::from_secs(SIGNAL_SLEEP_TIME_IN_SECS);
    let signaler = ConsoleSignal {};

    let mut status_cache : Option<Vec<StatusReport>> = None;
    spawn(move || {
        loop {
            let recv_result = status_channel.try_recv();

            match recv_result {
                Ok(v) => { status_cache = Some(v) },
                Err(_) => {}
            }

            match &status_cache {
                &Some(ref statuses) => {
                    for status in statuses.into_iter() {
                        signaler.signal_for_pin(&status).unwrap_or_else(|err| {
                            println!("Error during pin signaling {} : {}", &status.pin_id, err);
                        });
                    }
                },
                _ => {}
            }

            sleep(duration)
        }
    })
}