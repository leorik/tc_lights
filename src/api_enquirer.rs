extern crate hyper_native_tls;
extern crate serde_json;

use std::convert::From;
use std::fmt;
use std::error::Error;
use std::io::{Read, Error as IoError};

use super::hyper::client::{Client, IntoUrl};
use super::hyper::error::Error as HyperError;
use super::hyper::header::{Headers, Accept, qitem};
use super::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use super::hyper::net::HttpsConnector;
use super::hyper::status::StatusCode;
use self::hyper_native_tls::NativeTlsClient;

use self::serde_json::{Value as JsonValue, Error as JsonError};

use settings::ProjectSettings;

pub enum BuildStatus {
    Unknown,
    InProgress,
    Success,
    Failure
}

impl fmt::Display for BuildStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            BuildStatus::Unknown => "UNKNOWN".to_string(),
            BuildStatus::InProgress => "IN PROGRESS".to_string(),
            BuildStatus::Success => "SUCCESS".to_string(),
            BuildStatus::Failure => "FAILURE".to_string()
        });

        Ok(())
    }
}

pub struct Enquirer {
    client : Client
}

impl Enquirer {
    pub fn new() -> Enquirer {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        Enquirer { client: client }
    }

    pub fn query_for_project(&self, tc_url : &String, project : &ProjectSettings) -> Result<BuildStatus, EnquirerError> {
        let is_running = self.query_for_running_build(tc_url, project)?;

        if is_running == true {
            Ok(BuildStatus::InProgress)
        } else {
            self.query_for_last_build(tc_url, project)
        }
    }

    fn query_for_running_build(&self, tc_url: &String, project : &ProjectSettings) -> Result<bool, EnquirerError> {
        let running_build_query = format!("{}/guestAuth/app/rest/buildTypes/id:{}/builds/running:true",
                                          tc_url.to_string(), project.build_type_id.to_string());

        let json_header = generate_json_accept_headers();

        let mut response = &mut self.client.get(&running_build_query).headers(json_header).send()?;

        if response.status != StatusCode::NotFound {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn query_for_last_build(&self, tc_url: &String, project : &ProjectSettings) -> Result<BuildStatus, EnquirerError> {
        let last_build_query = format!("{}/guestAuth/app/rest/buildTypes/id:{}/builds/count:1",
                                          tc_url.to_string(), project.build_type_id.to_string());

        let json_header = generate_json_accept_headers();

        let mut response = &mut self.client.get(&last_build_query).headers(json_header).send()?;

        let mut response_body = String::new();
        response.read_to_string(&mut response_body)?;

        let response_value : JsonValue = serde_json::from_str(&response_body)?;

        let status_value = response_value["status"].as_str()
            .ok_or(EnquirerError::Generic { description: "Build status field in server response is not valid string".to_string() })?;

        match status_value.as_ref() {
            "SUCCESS" => Ok(BuildStatus::Success),
            "FAILURE" => Ok(BuildStatus::Failure),
            v @ _ => Err(EnquirerError::Generic { description: format!("Unexpected build status value: {}", v) })
        }
    }
}

fn generate_json_accept_headers() -> Headers {
    let mut json_header = Headers::new();
    json_header.set(
        Accept(vec![
            qitem(Mime(TopLevel::Application, SubLevel::Json,
                       vec![(Attr::Charset, Value::Utf8)])),
        ])
    );

    json_header
}

#[derive(Debug)]
pub enum  EnquirerError {
    HttpError { description: String, cause: HyperError },
    IoError { description: String, cause: IoError },
    JsonError { description: String, cause: JsonError },
    Generic { description: String }
}

impl Error for EnquirerError {
    fn description(&self) -> &str {
        match self {
            &EnquirerError::HttpError { ref description, .. } => &description,
            &EnquirerError::IoError { ref description, .. } => &description,
            &EnquirerError::JsonError { ref description, .. } => &description,
            &EnquirerError::Generic { ref description, .. } => &description
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &EnquirerError::HttpError { ref cause, .. } => Some(cause),
            &EnquirerError::IoError { ref cause, .. } => Some(cause),
            &EnquirerError::JsonError { ref cause, .. } => Some(cause),
            &EnquirerError::Generic { .. } => None
        }
    }
}

impl fmt::Display for EnquirerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<HyperError> for EnquirerError {
    fn from(hyper_error: HyperError) -> EnquirerError {
        EnquirerError::HttpError { description: "Error on HTTP request processing".to_string(), cause: hyper_error }
    }
}

impl From<IoError> for EnquirerError {
    fn from(io_error: IoError) -> EnquirerError {
        EnquirerError::IoError { description: "Error on response reading".to_string(), cause: io_error }
    }
}

impl From<JsonError> for EnquirerError {
    fn from(json_error: JsonError) -> EnquirerError {
        EnquirerError::JsonError { description: "Error on response deserialization".to_string(), cause: json_error }
    }
}


