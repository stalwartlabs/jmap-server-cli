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

use std::borrow::Cow;

use jmap_client::{
    client::Client,
    core::query::Filter,
    principal::{
        query::{self, Comparator},
        Principal, Property, Type,
    },
};
use prettytable::{Attr, Cell, Row, Table};

use super::{TableName, UnwrapResult};

pub fn email_to_id(client: &Client, ptype: Type, email: &str) -> String {
    let mut response = client
        .principal_query(
            Filter::and([query::Filter::ptype(ptype), query::Filter::email(email)]).into(),
            None::<Vec<_>>,
        )
        .unwrap_result("query principals");
    match response.ids().len() {
        1 => response.take_ids().pop().unwrap(),
        0 => {
            eprintln!("Error: No principal found with email '{}'.", email);
            std::process::exit(1);
        }
        _ => {
            eprintln!("Error: Multiple principals found with email '{}'.", email);
            std::process::exit(1);
        }
    }
}

pub fn list_principals(
    client: &Client,
    ptype: Type,
    filter: Option<String>,
    properties: &[Property],
) {
    let filter = if let Some(filter) = filter {
        Filter::and(vec![
            query::Filter::ptype(ptype),
            query::Filter::text(filter),
        ])
    } else {
        query::Filter::ptype(ptype).into()
    };

    let mut request = client.build();
    let query_ref = request
        .query_principal()
        .filter(filter)
        .sort([Comparator::email()])
        .result_reference();
    request
        .get_principal()
        .ids_ref(query_ref)
        .properties(properties.iter().cloned());
    let results = request
        .send()
        .unwrap_result("list principals")
        .unwrap_method_responses()
        .pop()
        .unwrap_or_else(|| {
            eprintln!("Error: Received an empty response from server.");
            std::process::exit(1);
        })
        .unwrap_get_principal()
        .unwrap_result("list principals")
        .take_list();

    if !results.is_empty() {
        // Build table
        let mut table = Table::new();
        table.add_row(Row::new(
            properties
                .iter()
                .map(|p| Cell::new(p.table_name()).with_style(Attr::Bold))
                .collect(),
        ));

        for principal in &results {
            table.add_row(Row::new(build_cells(client, principal, properties)));
        }

        eprintln!();
        table.printstd();
    }

    eprintln!(
        "\n\n{} record{} found.\n",
        results.len(),
        if results.len() == 1 { "" } else { "s" }
    );
}

pub fn display_principal(client: &Client, id: &str, properties: &[Property]) {
    if let Some(principal) = client
        .principal_get(id, properties.iter().cloned().into())
        .unwrap_result("fetch principal")
    {
        eprintln!();
        let mut table = Table::new();
        for (property, value) in properties
            .iter()
            .zip(build_cells(client, &principal, properties))
        {
            table.add_row(Row::new(vec![
                Cell::new(property.table_name()).with_style(Attr::Bold),
                value,
            ]));
        }
        table.printstd();
        eprintln!();
    } else {
        eprintln!("Entry not found.");
        std::process::exit(0);
    }
}

fn build_cells(client: &Client, principal: &Principal, properties: &[Property]) -> Vec<Cell> {
    let mut cells = Vec::with_capacity(properties.len());
    for property in properties.iter() {
        let value: Cow<str> = match property {
            Property::Id => principal.id().unwrap_or("").into(),
            Property::Name => principal.name().unwrap_or("").into(),
            Property::Description => principal.description().unwrap_or("").into(),
            Property::Email => principal.email().unwrap_or("").into(),
            Property::Timezone => principal.timezone().unwrap_or("").into(),
            Property::Capabilities => principal
                .capabilities()
                .map(|c| c.join(", ").into())
                .unwrap_or_else(|| "".into()),
            Property::Aliases => principal
                .aliases()
                .map(|c| c.join(", ").into())
                .unwrap_or_else(|| "".into()),
            Property::DKIM => principal
                .dkim()
                .map(|d| {
                    format!(
                        "selector {}, expiration {}.",
                        d.selector().unwrap_or("(none)"),
                        d.expiration().unwrap_or(0)
                    )
                    .into()
                })
                .unwrap_or_else(|| "".into()),
            Property::Quota => principal
                .quota()
                .map(|q| q.to_string().into())
                .unwrap_or_else(|| "".into()),
            Property::Picture => principal.picture().unwrap_or("").into(),
            Property::Members => {
                if let Some(members) = principal.members() {
                    let mut request = client.build();
                    request
                        .get_principal()
                        .ids(members)
                        .properties([Property::Email]);
                    request
                        .send_get_principal()
                        .unwrap_result("fetch principals")
                        .list()
                        .iter()
                        .filter_map(|p| p.email())
                        .collect::<Vec<_>>()
                        .join(", ")
                        .into()
                } else {
                    "".into()
                }
            }
            _ => unreachable!(),
        };

        cells.push(Cell::new(value.as_ref()));
    }
    cells
}
