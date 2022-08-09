use jmap_client::{
    client::Client,
    core::set::SetObject,
    principal::{Property, Type},
};

use crate::modules::{common::email_to_id, UnwrapResult};

use super::{
    cli::ListCommands,
    common::{display_principal, list_principals},
};

pub fn cmd_list(client: Client, command: ListCommands) {
    match command {
        ListCommands::Create {
            email,
            name,
            description,
        } => {
            let mut request = client.build();
            let create_request = request.set_principal().create();
            create_request.ptype(Type::List).email(&email).name(name);
            if description.is_some() {
                create_request.description(description);
            }
            let create_id = create_request.create_id().unwrap();
            request
                .send_set_principal()
                .unwrap_result("create list")
                .created(&create_id)
                .unwrap_result("create list");
            println!("List '{}' successfully created.", email);
        }
        ListCommands::Update {
            email,
            name,
            description,
        } => {
            let update_id = email_to_id(&client, Type::List, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);

            if let Some(name) = name {
                update_request.name(name);
            }
            if description.is_some() {
                update_request.description(description);
            }

            request
                .send_set_principal()
                .unwrap_result("update list")
                .updated(&update_id)
                .unwrap_result("update list");
            println!("List '{}' successfully updated.", email);
        }
        ListCommands::AddMembers { email, members } => {
            let update_id = email_to_id(&client, Type::List, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);

            for member in members {
                update_request.member(&email_to_id(&client, Type::Individual, &member), true);
            }

            request
                .send_set_principal()
                .unwrap_result("update list")
                .updated(&update_id)
                .unwrap_result("update list");

            println!("List '{}' successfully updated.", email);
        }
        ListCommands::RemoveMembers { email, members } => {
            let update_id = email_to_id(&client, Type::List, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);

            for member in members {
                update_request.member(&email_to_id(&client, Type::Individual, &member), false);
            }

            request
                .send_set_principal()
                .unwrap_result("update list")
                .updated(&update_id)
                .unwrap_result("update list");

            println!("List '{}' successfully updated.", email);
        }
        ListCommands::List { filter } => {
            list_principals(
                &client,
                Type::List,
                filter,
                &[Property::Email, Property::Name, Property::Description],
            );
        }
        ListCommands::Display { email } => {
            display_principal(
                &client,
                &email_to_id(&client, Type::List, &email),
                &[
                    Property::Email,
                    Property::Name,
                    Property::Description,
                    Property::Members,
                ],
            );
        }
    }
}
