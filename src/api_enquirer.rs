extern crate hyper;

use self::hyper::client;

pub enum BuildStatus {
    Unknown,
    InProgress,
    Success,
    Failure
}