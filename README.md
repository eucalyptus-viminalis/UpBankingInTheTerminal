# UpBankingInTheTerminal

A secure CLI for the [Up Banking API](https://developer.up.com.au/), built in Rust.

## Security

- **Rust** — memory-safe, compiled binary with no runtime dependencies
- **rustls** — pure-Rust TLS (no OpenSSL)
- **secrecy::SecretString** — API token is zeroized from memory on drop
- **0600 permissions** — config file is owner-read/write only
- **No token flags** — token is never passed as a CLI argument; loaded from env var or config file

## Installation

### Quick install (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/eucalyptus-viminalis/UpBankingInTheTerminal/main/scripts/install.sh | bash
```

This detects your OS and architecture, downloads the latest release binary, and installs it to `/usr/local/bin`.

To install to a custom location:

```bash
INSTALL_DIR=~/.local/bin curl -fsSL https://raw.githubusercontent.com/eucalyptus-viminalis/UpBankingInTheTerminal/main/scripts/install.sh | bash
```

### Download from GitHub Releases

Pre-built binaries for macOS (Intel + Apple Silicon), Linux, and Windows are available on the [Releases page](https://github.com/eucalyptus-viminalis/UpBankingInTheTerminal/releases).

### Build from source

```bash
git clone https://github.com/eucalyptus-viminalis/UpBankingInTheTerminal.git
cd UpBankingInTheTerminal
cargo build --release
# Binary is at ./target/release/upbank
```

## Uninstalling

```bash
curl -fsSL https://raw.githubusercontent.com/eucalyptus-viminalis/UpBankingInTheTerminal/main/scripts/uninstall.sh | bash
```

The uninstaller removes the binary and gives you the option to keep or remove your config directory (which contains your saved API token).

### Manual uninstall

```bash
rm -f /usr/local/bin/upbank

# Optional — remove config (contains your saved token)
rm -rf ~/.config/upbanking
```

## Getting Started

### 1. Get your Personal Access Token

You need a Personal Access Token (PAT) from Up to use this CLI.

1. Go to [api.up.com.au/getting_started](https://api.up.com.au/getting_started)
2. Follow the prompts to generate your token

Your token will look something like `up:yeah:aBcDeFgHiJkLmNoPqRsTuVwXyZ123456`

> **Keep this token safe.** Anyone with your token can read your account and transaction data. Don't share it, don't commit it to git, and don't paste it anywhere public.

### 2. Save your token

```bash
upbank config set-token "up:yeah:your-token-here"
```

This saves it to `~/.config/upbanking/config.toml` with restricted file permissions (only your user can read it).

### 3. Verify it works

```bash
upbank ping
```

You should see a success message with a emoji if everything is connected.

### Alternative: environment variable

If you prefer not to save the token to disk, you can set it as an environment variable instead. This takes priority over the config file:

```bash
export UP_API_TOKEN="up:yeah:your-token-here"
```

## Usage

All commands support `--json` for machine-readable output.

### Ping

```bash
upbank ping
```

### Accounts

```bash
# List all accounts
upbank accounts list

# Filter by type
upbank accounts list --account-type SAVER
upbank accounts list --account-type TRANSACTIONAL
upbank accounts list --account-type HOME_LOAN

# Filter by ownership
upbank accounts list --ownership-type JOINT

# Get a specific account
upbank accounts get <account-id>
```

### Transactions

```bash
# List all transactions
upbank transactions list

# Filter by status
upbank transactions list --status SETTLED
upbank transactions list --status HELD

# Filter by date range (RFC-3339)
upbank transactions list --since 2024-01-01T00:00:00Z --until 2024-12-31T23:59:59Z

# Filter by category or tag
upbank transactions list --category groceries
upbank transactions list --tag holidays

# Filter to a specific account
upbank transactions list --account <account-id>

# Combine filters
upbank transactions list --status SETTLED --category groceries --since 2024-06-01T00:00:00Z

# Get a specific transaction (detailed view)
upbank transactions get <transaction-id>
```

### Categories

```bash
# List all categories
upbank categories list

# List children of a parent category
upbank categories list --parent good-life

# Get a specific category
upbank categories get groceries
```

### Tags

```bash
upbank tags
upbank tags --page-size 50
```

### Attachments

```bash
# List all attachments
upbank attachments list

# Get attachment details (includes temporary download URL)
upbank attachments get <attachment-id>
```

### Webhooks

```bash
# List all webhooks
upbank webhooks list

# Get webhook details
upbank webhooks get <webhook-id>

# View delivery logs
upbank webhooks logs <webhook-id>
```

### JSON output

Add `--json` to any command for JSON output:

```bash
upbank --json accounts list
upbank --json transactions list --status SETTLED | jq '.[] | .amount'
```

## Pagination

List commands return the first page of results by default. Use `--page-size` to control how many results are returned (max varies by endpoint, typically up to 100):

```bash
upbank transactions list --page-size 100
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
