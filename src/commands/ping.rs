use crate::client::UpClient;
use crate::models::ping::PingResponse;

pub async fn run(client: &UpClient, json: bool) -> anyhow::Result<()> {
    let resp: PingResponse = client.get_raw("/util/ping").await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&resp.meta)?);
    } else {
        println!(
            "{} Authenticated successfully (id: {})",
            resp.meta.status_emoji, resp.meta.id
        );
    }

    Ok(())
}
