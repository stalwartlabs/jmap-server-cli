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
    core::{query::Filter, set::SetObject},
    principal::{query, Property, Type},
};

use crate::modules::{common::email_to_id, UnwrapResult};

use super::{
    cli::AccountCommands,
    common::{display_principal, list_principals},
};

pub fn cmd_account(client: Client, command: AccountCommands) {
    match command {
        AccountCommands::Create {
            email,
            password,
            name,
            description,
            quota,
            timezone,
            email_aliases,
        } => {
            // Create the domain if missing
            if let Some((_, domain)) = email.rsplit_once('@') {
                let domain_ = domain.to_ascii_lowercase();
                if client
                    .principal_query(
                        Filter::and([
                            query::Filter::ptype(Type::Domain),
                            query::Filter::domain_name(domain_),
                        ])
                        .into(),
                        None::<Vec<_>>,
                    )
                    .unwrap_result("query principals")
                    .ids()
                    .is_empty()
                {
                    // Create domain
                    client
                        .domain_create(domain)
                        .unwrap_result(&format!("Failed to create domain '{}'", domain));
                }
            } else {
                eprintln!("Invalid email address '{}'", email);
                std::process::exit(1);
            }

            let mut request = client.build();
            let create_request = request.set_principal().create();
            create_request
                .ptype(Type::Individual)
                .email(&email)
                .secret(password)
                .name(name);
            if description.is_some() {
                create_request.description(description);
            }
            if quota.is_some() {
                create_request.quota(quota);
            }
            if timezone.is_some() {
                create_request.timezone(timezone);
            }
            if email_aliases.is_some() {
                create_request.aliases(email_aliases);
            }
            let create_id = create_request.create_id().unwrap();
            request
                .send_set_principal()
                .unwrap_result("create account")
                .created(&create_id)
                .unwrap_result("create account");
            eprintln!("Account '{}' successfully created.", email);
        }
        AccountCommands::Update {
            email,
            password,
            name,
            description,
            quota,
            timezone,
        } => {
            let update_id = email_to_id(&client, Type::Individual, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);
            if let Some(password) = password {
                update_request.secret(password);
            }
            if let Some(name) = name {
                update_request.name(name);
            }
            if description.is_some() {
                update_request.description(description);
            }
            if let Some(quota) = quota {
                if quota > 0 {
                    update_request.quota(quota.into());
                } else {
                    update_request.quota(None);
                }
            }
            if timezone.is_some() {
                update_request.timezone(timezone);
            }
            request
                .send_set_principal()
                .unwrap_result("update account")
                .updated(&update_id)
                .unwrap_result("update account");
            eprintln!("Account '{}' successfully updated.", email);
        }
        AccountCommands::Delete { email } => {
            client
                .principal_destroy(&email_to_id(&client, Type::Individual, &email))
                .unwrap_result("delete account");
            eprintln!("Account '{}' successfully deleted.", email);
        }
        AccountCommands::List { filter } => {
            list_principals(
                &client,
                Type::Individual,
                filter,
                &[
                    Property::Email,
                    Property::Name,
                    Property::Description,
                    Property::Quota,
                ],
            );
        }
        AccountCommands::Display { email } => {
            display_principal(
                &client,
                &email_to_id(&client, Type::Individual, &email),
                &[
                    Property::Email,
                    Property::Name,
                    Property::Description,
                    Property::Quota,
                    Property::Timezone,
                    Property::Aliases,
                ],
            );
        }
        AccountCommands::AddAlias { email, aliases } => {
            let update_id = email_to_id(&client, Type::Individual, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);

            for alias in aliases {
                update_request.alias(&alias, true);
            }

            request
                .send_set_principal()
                .unwrap_result("update account")
                .updated(&update_id)
                .unwrap_result("update account");
            eprintln!("Account '{}' successfully updated.", email);
        }
        AccountCommands::RemoveAlias { email, aliases } => {
            let update_id = email_to_id(&client, Type::Individual, &email);
            let mut request = client.build();
            let update_request = request.set_principal().update(&update_id);

            for alias in aliases {
                update_request.alias(&alias, false);
            }

            request
                .send_set_principal()
                .unwrap_result("update account")
                .updated(&update_id)
                .unwrap_result("update account");
            eprintln!("Account '{}' successfully updated.", email);
        }
    }
}
