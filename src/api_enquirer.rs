extern crate hyper_native_tls;

use std::fmt;
use std::io::Read;

use super::hyper::client::{Client, IntoUrl};
use super::hyper::header::{Headers, Accept, qitem};
use super::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use super::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;
use settings::ProjectSettings;

pub enum BuildStatus {
    Unknown,
    InProgress,
    Success,
    Failure
}

impl fmt::Display for BuildStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            Unknown => "UNKNOWN".to_string(),
            InProgress => "IN PROGRESS".to_string(),
            Success => "SUCCESS".to_string(),
            Failure => "FAILURE".to_string()
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

    pub fn query_for_project(&self, tc_url : &String, project : &ProjectSettings) {
        let running_build_query = format!("{}/guest/app/rest/buildTypes/id:{}/builds/running:true",
                                          tc_url.to_string(), project.build_type_id.to_string());

        let json_header = generate_json_accept_headers();

        let mut response = String::new();
        let status = &self.client.get(&running_build_query).headers(json_header).send().unwrap()
            .read_to_string(&mut response).unwrap();
        println!("{}", response);
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
