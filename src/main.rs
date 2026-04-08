#![allow(dead_code)]

mod client;
mod commands;
mod config;
mod models;
mod output;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "upbank",
    about = "A secure CLI for the Up Banking API",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output as JSON instead of a table
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Check API connectivity and token validity
    Ping,

    /// Manage accounts
    #[command(subcommand)]
    Accounts(AccountsCmd),

    /// Browse and filter transactions
    #[command(subcommand)]
    Transactions(TransactionsCmd),

    /// View transaction categories
    #[command(subcommand)]
    Categories(CategoriesCmd),

    /// List tags
    Tags {
        /// Number of results per page
        #[arg(long)]
        page_size: Option<u8>,
    },

    /// View attachments
    #[command(subcommand)]
    Attachments(AttachmentsCmd),

    /// Manage webhooks
    #[command(subcommand)]
    Webhooks(WebhooksCmd),

    /// Manage CLI configuration
    #[command(subcommand)]
    Config(ConfigCmd),

    /// Remove upbank from your system
    Uninstall,
}

#[derive(Subcommand)]
enum AccountsCmd {
    /// List all accounts
    List {
        /// Filter by type: SAVER, TRANSACTIONAL, HOME_LOAN
        #[arg(long)]
        account_type: Option<String>,
        /// Filter by ownership: INDIVIDUAL, JOINT
        #[arg(long)]
        ownership_type: Option<String>,
        /// Number of results per page
        #[arg(long)]
        page_size: Option<u8>,
    },
    /// Get a specific account by ID
    Get {
        /// Account ID
        id: String,
    },
}

#[derive(Subcommand)]
enum TransactionsCmd {
    /// List transactions (optionally for a specific account)
    List {
        /// Filter to a specific account ID
        #[arg(long)]
        account: Option<String>,
        /// Filter by status: HELD, SETTLED
        #[arg(long)]
        status: Option<String>,
        /// Filter from date (RFC-3339, e.g. 2024-01-01T00:00:00Z)
        #[arg(long)]
        since: Option<String>,
        /// Filter to date (RFC-3339, e.g. 2024-12-31T23:59:59Z)
        #[arg(long)]
        until: Option<String>,
        /// Filter by category ID
        #[arg(long)]
        category: Option<String>,
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
        /// Number of results per page
        #[arg(long)]
        page_size: Option<u8>,
    },
    /// Get a specific transaction by ID
    Get {
        /// Transaction ID
        id: String,
    },
}

#[derive(Subcommand)]
enum CategoriesCmd {
    /// List all categories
    List {
        /// Filter by parent category ID
        #[arg(long)]
        parent: Option<String>,
    },
    /// Get a specific category by ID
    Get {
        /// Category ID (e.g. "groceries", "good-life")
        id: String,
    },
}

#[derive(Subcommand)]
enum AttachmentsCmd {
    /// List all attachments
    List {
        /// Number of results per page
        #[arg(long)]
        page_size: Option<u8>,
    },
    /// Get a specific attachment by ID (includes download URL)
    Get {
        /// Attachment ID
        id: String,
    },
}

#[derive(Subcommand)]
enum WebhooksCmd {
    /// List all webhooks
    List {
        /// Number of results per page
        #[arg(long)]
        page_size: Option<u8>,
    },
    /// Get a specific webhook by ID
    Get {
        /// Webhook ID
        id: String,
    },
    /// View delivery logs for a webhook
    Logs {
        /// Webhook ID
        id: String,
        /// Number of results per page
        #[arg(long)]
        page_size: Option<u8>,
    },
}

#[derive(Subcommand)]
enum ConfigCmd {
    /// Save your Up API token to the config file
    SetToken {
        /// Your Up Personal Access Token
        token: String,
    },
    /// Show the currently configured token (masked)
    ShowToken,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Commands that don't need a token
    match &cli.command {
        Commands::Config(cmd) => {
            let result = match cmd {
                ConfigCmd::SetToken { token } => commands::config_cmd::set_token(token),
                ConfigCmd::ShowToken => commands::config_cmd::show_token(),
            };
            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            return;
        }
        Commands::Uninstall => {
            if let Err(e) = commands::uninstall::run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            return;
        }
        _ => {}
    }

    // All other commands require authentication
    let token = match config::load_token() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let client = match client::UpClient::new(&token) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error creating API client: {}", e);
            std::process::exit(1);
        }
    };

    let result = match &cli.command {
        Commands::Ping => commands::ping::run(&client, cli.json).await,

        Commands::Accounts(cmd) => match cmd {
            AccountsCmd::List {
                account_type,
                ownership_type,
                page_size,
            } => {
                commands::accounts::list(
                    &client,
                    account_type.clone(),
                    ownership_type.clone(),
                    *page_size,
                    cli.json,
                )
                .await
            }
            AccountsCmd::Get { id } => commands::accounts::get(&client, id, cli.json).await,
        },

        Commands::Transactions(cmd) => match cmd {
            TransactionsCmd::List {
                account,
                status,
                since,
                until,
                category,
                tag,
                page_size,
            } => {
                commands::transactions::list(
                    &client,
                    account.clone(),
                    status.clone(),
                    since.clone(),
                    until.clone(),
                    category.clone(),
                    tag.clone(),
                    *page_size,
                    cli.json,
                )
                .await
            }
            TransactionsCmd::Get { id } => {
                commands::transactions::get(&client, id, cli.json).await
            }
        },

        Commands::Categories(cmd) => match cmd {
            CategoriesCmd::List { parent } => {
                commands::categories::list(&client, parent.clone(), cli.json).await
            }
            CategoriesCmd::Get { id } => {
                commands::categories::get(&client, id, cli.json).await
            }
        },

        Commands::Tags { page_size } => {
            commands::tags::list(&client, *page_size, cli.json).await
        }

        Commands::Attachments(cmd) => match cmd {
            AttachmentsCmd::List { page_size } => {
                commands::attachments::list(&client, *page_size, cli.json).await
            }
            AttachmentsCmd::Get { id } => {
                commands::attachments::get(&client, id, cli.json).await
            }
        },

        Commands::Webhooks(cmd) => match cmd {
            WebhooksCmd::List { page_size } => {
                commands::webhooks::list(&client, *page_size, cli.json).await
            }
            WebhooksCmd::Get { id } => commands::webhooks::get(&client, id, cli.json).await,
            WebhooksCmd::Logs { id, page_size } => {
                commands::webhooks::logs(&client, id, *page_size, cli.json).await
            }
        },

        Commands::Config(_) | Commands::Uninstall => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
