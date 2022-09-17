/*
 * Copyright (c) 2020-2022, Stalwart Labs Ltd.
 *
 * This file is part of the Stalwart Command Line Interface.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * in the LICENSE file at the top-level directory of this distribution.
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * You can be released from the requirements of the AGPLv3 license by
 * purchasing a commercial license. Please contact licensing@stalw.art
 * for more details.
*/

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
            eprintln!("Group '{}' successfully created.", email);
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
            eprintln!("Group '{}' successfully updated.", email);
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

            eprintln!("Group '{}' successfully updated.", email);
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

            eprintln!("Group '{}' successfully updated.", email);
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
