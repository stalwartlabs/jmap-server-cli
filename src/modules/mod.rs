use std::{collections::HashMap, fmt::Display, io::Read};

use jmap_client::principal::Property;

pub mod account;
pub mod cli;
pub mod common;
pub mod domain;
pub mod group;
pub mod import;
pub mod list;

pub trait UnwrapResult<T> {
    fn unwrap_result(self, action: &str) -> T;
}

impl<T> UnwrapResult<T> for Option<T> {
    fn unwrap_result(self, message: &str) -> T {
        match self {
            Some(result) => result,
            None => {
                println!("Failed to {}", message);
                std::process::exit(1);
            }
        }
    }
}

impl<T, E: Display> UnwrapResult<T> for Result<T, E> {
    fn unwrap_result(self, message: &str) -> T {
        match self {
            Ok(result) => result,
            Err(err) => {
                println!("Failed to {}: {}", message, err);
                std::process::exit(1);
            }
        }
    }
}

trait TableName {
    fn table_name(&self) -> &'static str;
}

impl TableName for Property {
    fn table_name(&self) -> &'static str {
        match self {
            Property::Id => "Id",
            Property::Type => "Type",
            Property::Name => "Name",
            Property::Description => "Description",
            Property::Email => "E-mail",
            Property::Timezone => "Timezone",
            Property::Capabilities => "Capabilities",
            Property::Aliases => "Aliases",
            Property::Secret => "Secret",
            Property::DKIM => "DKIM",
            Property::Quota => "Quota",
            Property::Picture => "Picture",
            Property::Members => "Members",
            Property::ACL => "ACL",
        }
    }
}

pub fn read_file(path: &str) -> Vec<u8> {
    if path == "-" {
        let mut stdin = std::io::stdin().lock();
        let mut raw_message = Vec::with_capacity(1024);
        let mut buf = [0; 1024];
        loop {
            let n = stdin.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            raw_message.extend_from_slice(&buf[..n]);
        }
        raw_message
    } else {
        std::fs::read(path).unwrap_or_else(|_| {
            println!("Failed to read file: {}", path);
            std::process::exit(1);
        })
    }
}

pub fn get(url: &str) -> HashMap<String, serde_json::Value> {
    serde_json::from_slice(
        &reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_default()
            .get(url)
            .send()
            .unwrap_result("send OAuth GET request")
            .bytes()
            .unwrap_result("fetch bytes"),
    )
    .unwrap_result("deserialize OAuth GET response")
}

pub fn post(url: &str, params: &HashMap<String, String>) -> HashMap<String, serde_json::Value> {
    serde_json::from_slice(
        &reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_default()
            .post(url)
            .form(params)
            .send()
            .unwrap_result("send OAuth POST request")
            .bytes()
            .unwrap_result("fetch bytes"),
    )
    .unwrap_result("deserialize OAuth POST response")
}

pub trait OAuthResponse {
    fn property(&self, name: &str) -> &str;
}

impl OAuthResponse for HashMap<String, serde_json::Value> {
    fn property(&self, name: &str) -> &str {
        self.get(name)
            .unwrap_result(&format!("find '{}' in OAuth response", name))
            .as_str()
            .unwrap_result(&format!("invalid '{}' value", name))
    }
}
