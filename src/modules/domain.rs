use std::fs;

use jmap_client::{
    client::Client,
    core::{query::Filter, set::SetObject},
    principal::{query, Property, Type, DKIM},
};

use crate::modules::UnwrapResult;

use super::{
    cli::DomainCommands,
    common::{display_principal, list_principals},
};

pub fn cmd_domain(client: Client, command: DomainCommands) {
    match command {
        DomainCommands::Create {
            name,
            description,
            dkim_cert,
            dkim_selector,
            dkim_expiration,
        } => {
            let mut request = client.build();
            let create_request = request.set_principal().create();
            create_request.ptype(Type::Domain).name(&name);
            if description.is_some() {
                create_request.description(description);
            }
            if let Some(dkim_cert) = dkim_cert {
                create_request.secret(
                    fs::read_to_string(dkim_cert).expect("Failed to read DKIM certificate file."),
                );
            }
            if dkim_selector.is_some() || dkim_expiration.is_some() {
                create_request.dkim(DKIM::new(dkim_selector, dkim_expiration.map(|s| s as i64)));
            }
            let create_id = create_request.create_id().unwrap();
            request
                .send_set_principal()
                .unwrap_result("create domain")
                .created(&create_id)
                .unwrap_result("create domain");
            println!("Domain '{}' successfully created.", name);
        }
        DomainCommands::Update {
            name,
            description,
            dkim_cert,
            dkim_selector,
            dkim_expiration,
        } => {
            let update_id = domain_to_id(&client, &name);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);
            if description.is_some() {
                update_request.description(description);
            }
            if let Some(dkim_cert) = dkim_cert {
                update_request.secret(
                    fs::read_to_string(dkim_cert).expect("Failed to read DKIM certificate file."),
                );
            }
            if dkim_selector.is_some() || dkim_expiration.is_some() {
                update_request.dkim(DKIM::new(dkim_selector, dkim_expiration.map(|s| s as i64)));
            }
            request
                .send_set_principal()
                .unwrap_result("update domain")
                .updated(&update_id)
                .unwrap_result("update domain");
            println!("Domain '{}' successfully updated.", name);
        }
        DomainCommands::Delete { name } => {
            client
                .principal_destroy(&domain_to_id(&client, &name))
                .unwrap_result("delete Domain");
            println!("Domain '{}' successfully deleted.", name);
        }
        DomainCommands::Display { name } => {
            display_principal(
                &client,
                &domain_to_id(&client, &name),
                &[Property::Name, Property::Description, Property::DKIM],
            );
        }
        DomainCommands::List { filter } => {
            list_principals(
                &client,
                Type::Domain,
                filter,
                &[Property::Name, Property::Description],
            );
        }
    }
}

pub fn domain_to_id(client: &Client, name: &str) -> String {
    let mut response = client
        .principal_query(
            Filter::and([
                query::Filter::ptype(Type::Domain),
                query::Filter::domain_name(name),
            ])
            .into(),
            None::<Vec<_>>,
        )
        .unwrap_result("query principals");
    match response.ids().len() {
        1 => response.take_ids().pop().unwrap(),
        0 => {
            println!("Error: No domain found with name '{}'.", name);
            std::process::exit(1);
        }
        _ => {
            println!("Error: Multiple domains found with name '{}'.", name);
            std::process::exit(1);
        }
    }
}
