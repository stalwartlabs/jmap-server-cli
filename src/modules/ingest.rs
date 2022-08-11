use jmap_client::client::Client;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

use super::{cli::IngestCommand, read_file, UnwrapResult};

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Dsn {
    pub to: String,
    pub status: DeliveryStatus,
    pub reason: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum DeliveryStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "notFound")]
    NotFound,
    #[serde(rename = "temporaryFailure")]
    TemporaryFailure,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum IngestResult {
    Dsn(Vec<Dsn>),
    Problem(serde_json::Value),
}

pub fn cmd_ingest(client: Client, command: IngestCommand, url: &str) {
    let raw_message = read_file(&command.path);
    let url = if let Some(from) = command.from {
        format!(
            "{}/ingest?from={},to={}",
            url,
            from,
            command.recipients.join(",")
        )
    } else {
        format!("{}/ingest?to={}", url, command.recipients.join(","))
    };

    let result = serde_json::from_slice::<IngestResult>(
        &reqwest::blocking::Client::builder()
            .default_headers(client.headers().clone())
            .build()
            .unwrap_or_default()
            .post(&url)
            .header(CONTENT_TYPE, "message/rfc822")
            .body(raw_message)
            .send()
            .unwrap_result("send request to JMAP server")
            .bytes()
            .unwrap_result("obtain response from JMAP server"),
    )
    .unwrap_result("parse JSON response");

    match result {
        IngestResult::Dsn(dsns) => {
            for dsn in dsns {
                let exit_code = match dsn.status {
                    DeliveryStatus::Success => continue,
                    DeliveryStatus::Failure => 77,  // EX_NOPERM
                    DeliveryStatus::NotFound => 67, // EX_NOUSER
                    DeliveryStatus::TemporaryFailure => 75, // EX_TEMPFAIL
                };
                println!(
                    "<{}>: {}",
                    dsn.to,
                    dsn.reason.unwrap_or_else(|| "Unknown error".to_string())
                );
                std::process::exit(exit_code);
            }
            std::process::exit(0);
        }
        IngestResult::Problem(response) => {
            println!(
                "Received unexpected response from server: {}",
                serde_json::to_string(&response).unwrap()
            );
            std::process::exit(1);
        }
    }
}
