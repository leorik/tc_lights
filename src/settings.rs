use std::io;

pub struct LightsSettings {
    teamcity_url : String,
    projects: Vec<ProjectSettings>
}

pub struct ProjectSettings {
    build_type_id : String,
    pin_id : String
}

pub fn read_settings() -> Result<LightsSettings, io::Error> {
    unimplemented!();
}