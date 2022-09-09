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

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(name = "stalwart-cli")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    /// JMAP server base URL
    #[clap(short, long)]
    pub url: String,
    /// Authentication credentials
    #[clap(short, long)]
    pub credentials: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage user accounts
    #[clap(subcommand)]
    Account(AccountCommands),

    /// Manage domains
    #[clap(subcommand)]
    Domain(DomainCommands),

    /// Manage mailing lists
    #[clap(subcommand)]
    List(ListCommands),

    /// Manage groups
    #[clap(subcommand)]
    Group(GroupCommands),

    /// Import accounts and domains
    #[clap(subcommand)]
    Import(ImportCommands),
}

#[derive(Subcommand)]
pub enum AccountCommands {
    /// Create a new user account
    Create {
        /// Login email address
        email: String,
        /// Password
        password: String,
        /// Account Name
        name: String,
        /// Account description
        #[clap(short, long)]
        description: Option<String>,
        /// Quota in bytes
        #[clap(short, long)]
        quota: Option<u32>,
        /// Timezone
        #[clap(short, long)]
        timezone: Option<String>,
        /// E-mail address aliases
        #[clap(short, long)]
        email_aliases: Option<Vec<String>>,
    },

    /// Update an existing user account
    Update {
        /// Account email address
        email: String,
        /// Update password
        #[clap(short, long)]
        password: Option<String>,
        /// Update account name
        #[clap(short, long)]
        name: Option<String>,
        /// Update account description
        #[clap(short, long)]
        description: Option<String>,
        /// Update quota in bytes
        #[clap(short, long)]
        quota: Option<u32>,
        /// Update timezone
        #[clap(short, long)]
        timezone: Option<String>,
    },

    /// Add e-mail aliases to a user account
    AddAlias {
        /// Account email address
        email: String,
        /// Aliases to add
        #[clap(required = true)]
        aliases: Vec<String>,
    },

    /// Add e-mail aliases to a user account
    RemoveAlias {
        /// Account email address
        email: String,
        /// Aliases to remove
        #[clap(required = true)]
        aliases: Vec<String>,
    },

    /// Delete an existing user account
    Delete {
        /// Account name to delete
        email: String,
    },

    /// Display an existing user account
    Display {
        /// Account name to display
        email: String,
    },

    /// List all user accounts
    List { filter: Option<String> },
}

#[derive(Subcommand)]
pub enum DomainCommands {
    /// Create a new domain
    Create {
        /// Domain name to create
        name: String,
        /// Description
        #[clap(short, long)]
        description: Option<String>,
        /// Path to DKIM private key.
        #[clap(short, long)]
        cert_dkim: Option<PathBuf>,
        /// DKIM selector
        #[clap(short, long)]
        selector_dkim: Option<String>,
        /// DKIM expiration (in seconds)
        #[clap(short, long)]
        expiration_dkim: Option<u64>,
    },

    /// Update an existing domain
    Update {
        /// Domain name to update
        name: String,
        /// Description
        #[clap(short, long)]
        description: Option<String>,
        /// Path to DKIM private key.
        #[clap(short, long)]
        cert_dkim: Option<PathBuf>,
        /// DKIM selector
        #[clap(short, long)]
        selector_dkim: Option<String>,
        /// DKIM expiration (in seconds)
        #[clap(short, long)]
        expiration_dkim: Option<u64>,
    },

    /// Delete an existing domain
    Delete {
        /// Domain name to delete
        name: String,
    },

    /// Display an existing domain
    Display {
        /// Domain name to display
        name: String,
    },

    /// List all domains
    List { filter: Option<String> },
}

#[derive(Subcommand)]
pub enum ListCommands {
    /// Create a new mailing list
    Create {
        /// List email address
        email: String,
        /// Name
        name: String,
        /// Description
        #[clap(short, long)]
        description: Option<String>,
    },

    /// Update an existing mailing list
    Update {
        /// List email address
        email: String,
        /// Name
        #[clap(short, long)]
        name: Option<String>,
        /// Description
        #[clap(short, long)]
        description: Option<String>,
    },

    /// Add members to a mailing list
    AddMembers {
        /// List email address
        email: String,
        /// Members to add
        #[clap(required = true)]
        members: Vec<String>,
    },

    /// Remove members from a mailing list
    RemoveMembers {
        /// List email address
        email: String,
        /// Members to remove
        #[clap(required = true)]
        members: Vec<String>,
    },

    /// List all mailing lists
    List { filter: Option<String> },

    /// Display an existing mailing list
    Display {
        /// Mailing list to display
        email: String,
    },
}

#[derive(Subcommand)]
pub enum GroupCommands {
    /// Create a group
    Create {
        /// Group email address
        email: String,
        /// Name
        name: String,
        /// Description
        #[clap(short, long)]
        description: Option<String>,
    },

    /// Update an existing group
    Update {
        /// Group email address
        email: String,
        /// Name
        #[clap(short, long)]
        name: Option<String>,
        /// Description
        #[clap(short, long)]
        description: Option<String>,
    },

    /// Add members to a group
    AddMembers {
        /// Group email address
        email: String,
        /// Members to add
        #[clap(required = true)]
        members: Vec<String>,
    },

    /// Remove members from a group
    RemoveMembers {
        /// Group email address
        email: String,
        /// Members to remove
        #[clap(required = true)]
        members: Vec<String>,
    },

    /// Display an existing group
    Display {
        /// Group email address to display
        email: String,
    },

    /// List all groups
    List { filter: Option<String> },
}

#[derive(Subcommand)]
pub enum ImportCommands {
    /// Bulk import user accounts
    Accounts {
        #[clap(short, long)]
        /// CSV file delimiter, defaults to ','
        delimiter: Option<String>,

        /// CSV has headers
        #[clap(short, long)]
        with_headers: bool,

        /// CSV column layout, default is 'email,secret,name,description,quota,timezone'
        #[clap(short, long)]
        column_layout: Option<String>,

        /// Do not create domain names
        #[clap(short, long)]
        no_domains: bool,

        /// Path to the CSV file, or '-' for stdin
        path: String,
    },

    /// Import messages and folders
    Messages {
        #[clap(arg_enum)]
        #[clap(short, long)]
        format: MailboxFormat,

        /// Number of threads to use for message import, defaults to the number of CPUs.
        #[clap(short, long)]
        num_threads: Option<usize>,

        /// Account email to import messages into
        email: String,

        /// Path to the mailbox to import, or '-' for stdin (stdin only supported for mbox)
        path: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MailboxFormat {
    /// Mbox format
    Mbox,
    /// Maildir and Maildir++ formats
    Maildir,
    /// Maildir with hierarchical folders (i.e. Dovecot)
    MaildirNested,
}
