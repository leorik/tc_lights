#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate hyper;

mod api_enquirer;
mod settings;

fn main() {
    let settings = settings::read_settings().unwrap();

//    println!("{}", &api_enquirer::BuildStatus::Unknown.to_string());

    let e = api_enquirer::Enquirer::new();

    let r = e.query_for_project(&settings.teamcity_url, &settings.projects[0]);

    println!("{}", r.unwrap());
}
