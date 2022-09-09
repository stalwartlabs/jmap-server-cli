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
            cert_dkim,
            selector_dkim,
            expiration_dkim,
        } => {
            let mut request = client.build();
            let create_request = request.set_principal().create();
            create_request.ptype(Type::Domain).name(&name);
            if description.is_some() {
                create_request.description(description);
            }
            if let Some(cert_dkim) = cert_dkim {
                create_request.secret(
                    fs::read_to_string(cert_dkim).unwrap_result("read DKIM certificate file."),
                );
            }
            if selector_dkim.is_some() || expiration_dkim.is_some() {
                create_request.dkim(DKIM::new(selector_dkim, expiration_dkim.map(|s| s as i64)));
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
            cert_dkim,
            selector_dkim,
            expiration_dkim,
        } => {
            let update_id = domain_to_id(&client, &name);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);
            if description.is_some() {
                update_request.description(description);
            }
            if let Some(cert_dkim) = cert_dkim {
                update_request.secret(
                    fs::read_to_string(cert_dkim).unwrap_result("read DKIM certificate file."),
                );
            }
            if selector_dkim.is_some() || expiration_dkim.is_some() {
                update_request.dkim(DKIM::new(selector_dkim, expiration_dkim.map(|s| s as i64)));
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
