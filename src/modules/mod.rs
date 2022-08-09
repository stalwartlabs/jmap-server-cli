use jmap_client::principal::Property;

pub mod account;
pub mod cli;
pub mod common;
pub mod domain;
pub mod group;
pub mod ingest;
pub mod list;

trait UnwrapResult<T> {
    fn unwrap_result(self, action: &str) -> T;
}

impl<T> UnwrapResult<T> for jmap_client::Result<T> {
    fn unwrap_result(self, action: &str) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                println!("Failed to {}: {}", action, err);
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
