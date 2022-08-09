use jmap_client::{
    client::Client,
    core::set::SetObject,
    principal::{Property, Type},
};

use crate::modules::{common::email_to_id, UnwrapResult};

use super::{
    cli::GroupCommands,
    common::{display_principal, list_principals},
};

pub fn cmd_group(client: Client, command: GroupCommands) {
    match command {
        GroupCommands::Create {
            email,
            name,
            description,
        } => {
            let mut request = client.build();
            let create_request = request.set_principal().create();
            create_request.ptype(Type::Group).email(&email).name(name);
            if description.is_some() {
                create_request.description(description);
            }
            let create_id = create_request.create_id().unwrap();
            request
                .send_set_principal()
                .unwrap_result("create group")
                .created(&create_id)
                .unwrap_result("create group");
            println!("Group '{}' successfully created.", email);
        }
        GroupCommands::Update {
            email,
            name,
            description,
        } => {
            let update_id = email_to_id(&client, Type::Group, &email);
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
                .unwrap_result("update group")
                .updated(&update_id)
                .unwrap_result("update group");
            println!("Group '{}' successfully updated.", email);
        }
        GroupCommands::AddMembers { email, members } => {
            let update_id = email_to_id(&client, Type::Group, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);

            for member in members {
                update_request.member(&email_to_id(&client, Type::Individual, &member), true);
            }

            request
                .send_set_principal()
                .unwrap_result("update group")
                .updated(&update_id)
                .unwrap_result("update group");

            println!("Group '{}' successfully updated.", email);
        }
        GroupCommands::RemoveMembers { email, members } => {
            let update_id = email_to_id(&client, Type::Group, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);

            for member in members {
                update_request.member(&email_to_id(&client, Type::Individual, &member), false);
            }

            request
                .send_set_principal()
                .unwrap_result("update group")
                .updated(&update_id)
                .unwrap_result("update group");

            println!("Group '{}' successfully updated.", email);
        }
        GroupCommands::List { filter } => {
            list_principals(
                &client,
                Type::Group,
                filter,
                &[Property::Email, Property::Name, Property::Description],
            );
        }
        GroupCommands::Display { email } => {
            display_principal(
                &client,
                &email_to_id(&client, Type::Group, &email),
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
