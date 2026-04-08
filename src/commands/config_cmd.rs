use secrecy::{ExposeSecret, SecretString};

use crate::config;

pub fn set_token(token: &str) -> anyhow::Result<()> {
    let secret = SecretString::from(token.to_string());
    config::save_token(&secret)?;
    println!(
        "Token saved ({}). Config file created with restricted permissions (0600).",
        config::mask_token(token)
    );
    Ok(())
}

pub fn show_token() -> anyhow::Result<()> {
    match config::load_token() {
        Ok(token) => {
            let masked = config::mask_token(token.expose_secret());
            println!("Current token: {}", masked);
        }
        Err(e) => {
            println!("No token configured: {}", e);
        }
    }
    Ok(())
}
