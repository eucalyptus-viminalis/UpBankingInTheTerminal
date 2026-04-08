use serde::Serialize;
use tabled::Tabled;

use crate::client::UpClient;
use crate::models::common::PaginatedResponse;
use crate::models::webhooks::{WebhookLogResource, WebhookResource};
use crate::output;

#[derive(Tabled, Serialize)]
struct WebhookRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "URL")]
    url: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Created")]
    created_at: String,
}

impl From<&WebhookResource> for WebhookRow {
    fn from(w: &WebhookResource) -> Self {
        Self {
            id: w.id.clone(),
            url: output::truncate(&w.attributes.url, 40),
            description: w
                .attributes
                .description
                .clone()
                .unwrap_or_default(),
            created_at: w.attributes.created_at.clone(),
        }
    }
}

#[derive(Tabled, Serialize)]
struct WebhookDetail {
    #[tabled(rename = "Field")]
    field: String,
    #[tabled(rename = "Value")]
    value: String,
}

#[derive(Tabled, Serialize)]
struct WebhookLogRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Status Code")]
    status_code: String,
    #[tabled(rename = "Body")]
    body: String,
    #[tabled(rename = "Created")]
    created_at: String,
}

impl From<&WebhookLogResource> for WebhookLogRow {
    fn from(l: &WebhookLogResource) -> Self {
        Self {
            id: l.id.clone(),
            status_code: l
                .attributes
                .status_code
                .map(|c| c.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            body: l
                .attributes
                .body
                .as_deref()
                .map(|b| output::truncate(b, 50))
                .unwrap_or_default(),
            created_at: l.attributes.created_at.clone(),
        }
    }
}

pub async fn list(
    client: &UpClient,
    page_size: Option<u8>,
    json: bool,
) -> anyhow::Result<()> {
    let mut params = Vec::new();
    if let Some(ps) = page_size {
        params.push(("page[size]".to_string(), ps.to_string()));
    }

    let resp: PaginatedResponse<WebhookResource> =
        client.get_many("/webhooks", &params).await?;
    let rows: Vec<WebhookRow> = resp.data.iter().map(WebhookRow::from).collect();
    output::print_table(&rows, json);
    Ok(())
}

pub async fn get(client: &UpClient, id: &str, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_one::<WebhookResource>(&format!("/webhooks/{}", id))
        .await?;
    let w = &resp.data;

    if json {
        println!("{}", serde_json::to_string_pretty(w)?);
        return Ok(());
    }

    let details = vec![
        WebhookDetail {
            field: "ID".into(),
            value: w.id.clone(),
        },
        WebhookDetail {
            field: "URL".into(),
            value: w.attributes.url.clone(),
        },
        WebhookDetail {
            field: "Description".into(),
            value: w.attributes.description.clone().unwrap_or_default(),
        },
        WebhookDetail {
            field: "Created".into(),
            value: w.attributes.created_at.clone(),
        },
    ];

    output::print_table(&details, false);
    Ok(())
}

pub async fn logs(
    client: &UpClient,
    webhook_id: &str,
    page_size: Option<u8>,
    json: bool,
) -> anyhow::Result<()> {
    let mut params = Vec::new();
    if let Some(ps) = page_size {
        params.push(("page[size]".to_string(), ps.to_string()));
    }

    let resp: PaginatedResponse<WebhookLogResource> = client
        .get_many(&format!("/webhooks/{}/logs", webhook_id), &params)
        .await?;
    let rows: Vec<WebhookLogRow> = resp.data.iter().map(WebhookLogRow::from).collect();
    output::print_table(&rows, json);
    Ok(())
}
