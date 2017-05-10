extern crate serde_json;

use std::io;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;

use super::serde_derive;

#[derive(Serialize, Deserialize)]
pub struct LightsSettings {
    pub teamcity_url: String,
    pub projects: Vec<ProjectSettings>
}

#[derive(Serialize, Deserialize)]
pub struct ProjectSettings {
    pub build_type_id: String,
    pub pin_id: String
}

pub fn read_settings() -> Result<LightsSettings, io::Error> {
    let json_settings = read_settings_file().unwrap();

    let settings : LightsSettings = serde_json::from_str(&json_settings).unwrap();

    Ok(settings)
}

fn read_settings_file() -> Result<String, io::Error> {
    let file = File::open("lights.json")?;
    let mut reader = BufReader::new(file);
    let mut json_contents = String::new();
    reader.read_to_string(&mut json_contents)?;

    Ok(json_contents)
}