use std::{fmt::Display, io::Read};

use jmap_client::principal::Property;

pub mod account;
pub mod cli;
pub mod common;
pub mod domain;
pub mod group;
pub mod import;
pub mod ingest;
pub mod list;

trait UnwrapResult<T> {
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
