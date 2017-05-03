#[macro_use]
extern crate serde_derive;

mod api_enquirer;
mod settings;

fn main() {
    let settings = settings::read_settings();
}
