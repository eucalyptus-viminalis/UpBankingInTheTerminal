use std::io::Write;

const REPO: &str = "eucalyptus-viminalis/UpBankingInTheTerminal";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn run() -> anyhow::Result<()> {
    println!("Current version: v{}", CURRENT_VERSION);
    println!("Checking for updates...");

    // Fetch latest release tag from GitHub API
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()?;

    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);
    let resp = client
        .get(&url)
        .header("User-Agent", "upbank-cli")
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Failed to check for updates (HTTP {})", resp.status());
    }

    let body: serde_json::Value = resp.json().await?;
    let latest_tag = body["tag_name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Could not parse latest release"))?;

    let latest_version = latest_tag.trim_start_matches('v');

    if latest_version == CURRENT_VERSION {
        println!("Already up to date!");
        return Ok(());
    }

    println!("New version available: v{}", latest_version);
    print!("Update now? (y/N) ");
    std::io::stdout().flush()?;

    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    if !line.trim().eq_ignore_ascii_case("y") {
        println!("Cancelled.");
        return Ok(());
    }

    // Detect platform
    let target = detect_target()?;
    let asset_name = format!("upbank-{}", target);
    let archive = format!("{}.tar.gz", asset_name);

    let download_url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        REPO, latest_tag, archive
    );

    println!("Downloading {}...", archive);

    let binary_data = client
        .get(&download_url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    // Extract and replace binary
    let current_exe = std::env::current_exe()?;
    let tmp_dir = std::env::temp_dir().join("upbank-update");
    std::fs::create_dir_all(&tmp_dir)?;

    let archive_path = tmp_dir.join(&archive);
    std::fs::write(&archive_path, &binary_data)?;

    // Extract tar.gz
    let output = std::process::Command::new("tar")
        .args(["xzf", archive_path.to_str().unwrap(), "-C", tmp_dir.to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to extract archive");
    }

    let new_binary = tmp_dir.join("upbank");
    if !new_binary.exists() {
        anyhow::bail!("Binary not found in archive");
    }

    // Replace the current binary
    match std::fs::copy(&new_binary, &current_exe) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("Permission denied. Try: sudo upbank update");
            std::process::exit(1);
        }
        Err(e) => return Err(e.into()),
    }

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);

    println!("Updated to v{}!", latest_version);
    Ok(())
}

fn detect_target() -> anyhow::Result<&'static str> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match (os, arch) {
        ("macos", "x86_64") => Ok("x86_64-apple-darwin"),
        ("macos", "aarch64") => Ok("aarch64-apple-darwin"),
        ("linux", "x86_64") => Ok("x86_64-linux-gnu"),
        _ => anyhow::bail!("Unsupported platform: {} {}", os, arch),
    }
}
