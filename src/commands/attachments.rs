use serde::Serialize;
use tabled::Tabled;

use crate::client::UpClient;
use crate::models::attachments::AttachmentResource;
use crate::models::common::PaginatedResponse;
use crate::output;

#[derive(Tabled, Serialize)]
struct AttachmentRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Type")]
    content_type: String,
    #[tabled(rename = "Extension")]
    extension: String,
    #[tabled(rename = "Transaction")]
    transaction: String,
    #[tabled(rename = "Created")]
    created_at: String,
}

impl From<&AttachmentResource> for AttachmentRow {
    fn from(a: &AttachmentResource) -> Self {
        let transaction = a
            .relationships
            .as_ref()
            .and_then(|r| r.transaction.as_ref())
            .and_then(|t| t.data.as_ref())
            .map(|d| output::truncate(&d.id, 16))
            .unwrap_or_default();

        Self {
            id: output::truncate(&a.id, 16),
            content_type: a
                .attributes
                .file_content_type
                .clone()
                .unwrap_or_default(),
            extension: a.attributes.file_extension.clone().unwrap_or_default(),
            transaction,
            created_at: a.attributes.created_at.clone().unwrap_or_default(),
        }
    }
}

#[derive(Tabled, Serialize)]
struct AttachmentDetail {
    #[tabled(rename = "Field")]
    field: String,
    #[tabled(rename = "Value")]
    value: String,
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

    let resp: PaginatedResponse<AttachmentResource> =
        client.get_many("/attachments", &params).await?;
    let rows: Vec<AttachmentRow> = resp.data.iter().map(AttachmentRow::from).collect();
    output::print_table(&rows, json);

    if let Some(links) = &resp.links {
        if links.next.is_some() && !json {
            println!("\n(more results available — use --page-size to adjust)");
        }
    }

    Ok(())
}

pub async fn get(client: &UpClient, id: &str, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_one::<AttachmentResource>(&format!("/attachments/{}", id))
        .await?;
    let a = &resp.data;

    if json {
        println!("{}", serde_json::to_string_pretty(a)?);
        return Ok(());
    }

    let mut details = vec![
        AttachmentDetail {
            field: "ID".into(),
            value: a.id.clone(),
        },
        AttachmentDetail {
            field: "Content Type".into(),
            value: a.attributes.file_content_type.clone().unwrap_or_default(),
        },
        AttachmentDetail {
            field: "Extension".into(),
            value: a.attributes.file_extension.clone().unwrap_or_default(),
        },
        AttachmentDetail {
            field: "Created".into(),
            value: a.attributes.created_at.clone().unwrap_or_default(),
        },
    ];

    if let Some(url) = &a.attributes.file_url {
        details.push(AttachmentDetail {
            field: "Download URL".into(),
            value: url.clone(),
        });
    }
    if let Some(expires) = &a.attributes.file_url_expires_at {
        details.push(AttachmentDetail {
            field: "URL Expires".into(),
            value: expires.clone(),
        });
    }

    if let Some(rels) = &a.relationships {
        if let Some(txn) = &rels.transaction {
            if let Some(data) = &txn.data {
                details.push(AttachmentDetail {
                    field: "Transaction".into(),
                    value: data.id.clone(),
                });
            }
        }
    }

    output::print_table(&details, false);
    Ok(())
}
