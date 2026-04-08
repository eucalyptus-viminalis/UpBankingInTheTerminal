use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub fn run() -> anyhow::Result<()> {
    // Find the binary path
    let binary_path = std::env::current_exe()?;

    // Config directory
    let config_dir = dirs::home_dir()
        .map(|home| home.join(".config").join("upbanking"))
        .unwrap_or_default();

    println!("This will remove:");
    println!("  Binary: {}", binary_path.display());
    if config_dir.exists() {
        println!("  Config: {} (will ask separately)", config_dir.display());
    }
    println!();

    if !confirm("Remove upbank binary?")? {
        println!("Cancelled.");
        return Ok(());
    }

    // Self-delete the binary
    remove_binary(&binary_path)?;
    println!("Removed binary.");

    // Ask about config
    if config_dir.exists() {
        println!();
        println!("  Config directory contains your saved API token.");
        if confirm("Remove config directory?")? {
            std::fs::remove_dir_all(&config_dir)?;
            println!("Removed config directory.");
        } else {
            println!("Kept config directory.");
        }
    }

    println!();
    println!("Uninstall complete.");
    Ok(())
}

fn confirm(prompt: &str) -> anyhow::Result<bool> {
    print!("{} (y/N) ", prompt);
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim().eq_ignore_ascii_case("y"))
}

fn remove_binary(path: &PathBuf) -> anyhow::Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!("Permission denied. Try running with sudo:");
            eprintln!("  sudo upbank uninstall");
            std::process::exit(1);
        }
        Err(e) => Err(e.into()),
    }
}
