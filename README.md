# UpBankingInTheTerminal

A secure CLI for the [Up Banking API](https://developer.up.com.au/), built in Rust.

## Security

- **Rust** — memory-safe, compiled binary with no runtime dependencies
- **rustls** — pure-Rust TLS (no OpenSSL)
- **secrecy::SecretString** — API token is zeroized from memory on drop
- **0600 permissions** — config file is owner-read/write only
- **No token flags** — token is never passed as a CLI argument; loaded from env var or config file

## Installation

```bash
# Clone and build
cd packages/UpBankingInTheTerminal
cargo build --release

# Binary is at:
./target/release/upbanking-in-the-terminal
```

Optionally alias it:

```bash
alias upbank="$(pwd)/target/release/upbanking-in-the-terminal"
```

## Authentication

Token resolution is tiered (env var takes priority):

1. `UP_API_TOKEN` environment variable
2. Config file at `~/.config/UpBankingInTheTerminal/config.toml`

```bash
# Option 1: env var
export UP_API_TOKEN="up:yeah:xxxxxxx"

# Option 2: save to config file (created with 0600 permissions)
upbanking-in-the-terminal config set-token "up:yeah:xxxxxxx"

# View current token (masked)
upbanking-in-the-terminal config show-token
```

Generate a Personal Access Token in the Up app under **Data Sharing > Personal Access Token**, or at [api.up.com.au](https://api.up.com.au).

## Usage

All commands support `--json` for machine-readable output.

### Ping

```bash
upbanking-in-the-terminal ping
```

### Accounts

```bash
# List all accounts
upbanking-in-the-terminal accounts list

# Filter by type
upbanking-in-the-terminal accounts list --account-type SAVER
upbanking-in-the-terminal accounts list --account-type TRANSACTIONAL
upbanking-in-the-terminal accounts list --account-type HOME_LOAN

# Filter by ownership
upbanking-in-the-terminal accounts list --ownership-type JOINT

# Get a specific account
upbanking-in-the-terminal accounts get <account-id>
```

### Transactions

```bash
# List all transactions
upbanking-in-the-terminal transactions list

# Filter by status
upbanking-in-the-terminal transactions list --status SETTLED
upbanking-in-the-terminal transactions list --status HELD

# Filter by date range (RFC-3339)
upbanking-in-the-terminal transactions list --since 2024-01-01T00:00:00Z --until 2024-12-31T23:59:59Z

# Filter by category or tag
upbanking-in-the-terminal transactions list --category groceries
upbanking-in-the-terminal transactions list --tag holidays

# Filter to a specific account
upbanking-in-the-terminal transactions list --account <account-id>

# Combine filters
upbanking-in-the-terminal transactions list --status SETTLED --category groceries --since 2024-06-01T00:00:00Z

# Get a specific transaction (detailed view)
upbanking-in-the-terminal transactions get <transaction-id>
```

### Categories

```bash
# List all categories
upbanking-in-the-terminal categories list

# List children of a parent category
upbanking-in-the-terminal categories list --parent good-life

# Get a specific category
upbanking-in-the-terminal categories get groceries
```

### Tags

```bash
upbanking-in-the-terminal tags
upbanking-in-the-terminal tags --page-size 50
```

### Attachments

```bash
# List all attachments
upbanking-in-the-terminal attachments list

# Get attachment details (includes temporary download URL)
upbanking-in-the-terminal attachments get <attachment-id>
```

### Webhooks

```bash
# List all webhooks
upbanking-in-the-terminal webhooks list

# Get webhook details
upbanking-in-the-terminal webhooks get <webhook-id>

# View delivery logs
upbanking-in-the-terminal webhooks logs <webhook-id>
```

### JSON output

Add `--json` to any command for JSON output:

```bash
upbanking-in-the-terminal --json accounts list
upbanking-in-the-terminal --json transactions list --status SETTLED | jq '.[] | .amount'
```

## Pagination

List commands return the first page of results by default. Use `--page-size` to control how many results are returned (max varies by endpoint, typically up to 100):

```bash
upbanking-in-the-terminal transactions list --page-size 100
```

## Project Structure

```
src/
  main.rs              CLI definition and entrypoint
  config.rs            Tiered auth (env var -> config file)
  client.rs            Up API HTTP client (reqwest + rustls)
  output.rs            Table/JSON output formatting
  commands/
    ping.rs            Ping command
    accounts.rs        Accounts list/get
    transactions.rs    Transactions list/get with filters
    categories.rs      Categories list/get
    tags.rs            Tags list
    attachments.rs     Attachments list/get
    webhooks.rs        Webhooks list/get/logs
    config_cmd.rs      Token management
  models/
    common.rs          Shared types (money, pagination, errors)
    accounts.rs        Account resource types
    transactions.rs    Transaction resource types
    categories.rs      Category resource types
    tags.rs            Tag resource types
    attachments.rs     Attachment resource types
    webhooks.rs        Webhook resource types
    ping.rs            Ping response type
```
