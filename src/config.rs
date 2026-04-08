use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("No API token found. Set UP_API_TOKEN env var or run `upbank config set-token`")]
    NoToken,
    #[error("Failed to read config file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseConfig(#[from] toml::de::Error),
}

#[derive(Deserialize)]
struct ConfigFile {
    token: Option<String>,
}

fn config_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".config").join("upbanking"))
}

fn config_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.toml"))
}

/// Load the API token using tiered resolution:
/// 1. UP_API_TOKEN environment variable (highest priority)
/// 2. Config file (platform-specific, via `directories` crate)
pub fn load_token() -> Result<SecretString, ConfigError> {
    // Tier 1: Environment variable
    if let Ok(token) = std::env::var("UP_API_TOKEN") {
        if !token.is_empty() {
            return Ok(SecretString::from(token));
        }
    }

    // Tier 2: Config file
    if let Some(path) = config_path() {
        if path.exists() {
            let contents = fs::read_to_string(&path)?;
            let config: ConfigFile = toml::from_str(&contents)?;
            if let Some(token) = config.token {
                if !token.is_empty() {
                    return Ok(SecretString::from(token));
                }
            }
        }
    }

    Err(ConfigError::NoToken)
}

/// Save the API token to the config file with restricted permissions.
pub fn save_token(token: &SecretString) -> Result<(), ConfigError> {
    let dir = config_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Could not determine config directory")
    })?;

    fs::create_dir_all(&dir)?;

    let path = dir.join("config.toml");
    let contents = format!("token = \"{}\"\n", token.expose_secret());

    // Write file with restricted permissions (owner-only read/write)
    {
        let mut opts = fs::OpenOptions::new();
        opts.write(true).create(true).truncate(true);
        #[cfg(unix)]
        opts.mode(0o600);
        let mut file = opts.open(&path)?;
        file.write_all(contents.as_bytes())?;
    }

    Ok(())
}

/// Mask a token for display, showing only the last 4 characters.
pub fn mask_token(token: &str) -> String {
    if token.len() <= 4 {
        return "****".to_string();
    }
    let visible = &token[token.len() - 4..];
    format!("****{}", visible)
}
