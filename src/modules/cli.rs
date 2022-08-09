use clap::{Args, Parser, Subcommand};
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

    /// Ingest an e-mail message
    Ingest(IngestCommand),
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
        dkim_cert: Option<PathBuf>,
        /// DKIM selector
        #[clap(short, long)]
        dkim_selector: Option<String>,
        /// DKIM expiration (in seconds)
        #[clap(short, long)]
        dkim_expiration: Option<u64>,
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
        dkim_cert: Option<PathBuf>,
        /// DKIM selector
        #[clap(short, long)]
        dkim_selector: Option<String>,
        /// DKIM expiration (in seconds)
        #[clap(short, long)]
        dkim_expiration: Option<u64>,
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

#[derive(Args)]
pub struct IngestCommand {
    /// Path to the message file, or '-' for stdin
    pub path: String,

    #[clap(short, long)]
    /// SMTP MAIL FROM address
    pub from: Option<String>,

    /// Deliver the message to the specified recipients
    #[clap(required = true)]
    pub recipients: Vec<String>,
}
