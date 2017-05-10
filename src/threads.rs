use std::thread::{spawn, sleep, JoinHandle};
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::Sender;

use api_enquirer::{Enquirer, StatusReport, BuildStatus};
use settings::LightsSettings;

const ENQUIRER_SLEEP_TIME_IN_SECS: u64 = 30;

pub fn run_enquirer_thread(settings: Arc<LightsSettings>, status_channel : Sender<Vec<StatusReport>>) -> JoinHandle<()> {
    let enquirer = Enquirer::new();

    spawn(move || {
        loop {
            println!("loop");
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

            sleep(Duration::from_secs(ENQUIRER_SLEEP_TIME_IN_SECS));
        }
    })
}