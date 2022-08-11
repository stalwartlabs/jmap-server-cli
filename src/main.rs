use clap::Parser;
use jmap_client::client::Client;
use modules::{
    account::cmd_account,
    cli::{Cli, Commands},
    domain::cmd_domain,
    group::cmd_group,
    import::cmd_import,
    ingest::cmd_ingest,
    list::cmd_list,
};

pub mod modules;

// main function
fn main() {
    let args = Cli::parse();
    let credentials = if let Some(credentials) = args.credentials {
        credentials
    } else {
        rpassword::prompt_password("Credentials: ").unwrap()
    };

    let (account, secret) = if let Some((account, secret)) = credentials.split_once(':') {
        (account, secret)
    } else if matches!(&args.command, Commands::Ingest { .. }) {
        ("ingest", credentials.as_str())
    } else {
        ("admin", credentials.as_str())
    };

    let client = Client::new()
        .credentials((account, secret))
        .connect(&format!("{}/.well-known/jmap", args.url))
        .unwrap_or_else(|err| {
            println!(
                "Failed to connect to JMAP server using account '{}': {}.",
                account, err
            );
            std::process::exit(1);
        });

    match args.command {
        Commands::Account(command) => cmd_account(client, command),
        Commands::Domain(command) => cmd_domain(client, command),
        Commands::List(command) => cmd_list(client, command),
        Commands::Group(command) => cmd_group(client, command),
        Commands::Ingest(command) => cmd_ingest(client, command, &args.url),
        Commands::Import(command) => cmd_import(client, command),
    }
}
