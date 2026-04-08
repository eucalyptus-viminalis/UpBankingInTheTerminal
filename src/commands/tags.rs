use serde::Serialize;
use tabled::Tabled;

use crate::client::UpClient;
use crate::models::common::PaginatedResponse;
use crate::models::tags::TagResource;
use crate::output;

#[derive(Tabled, Serialize)]
struct TagRow {
    #[tabled(rename = "Tag")]
    id: String,
}

impl From<&TagResource> for TagRow {
    fn from(t: &TagResource) -> Self {
        Self { id: t.id.clone() }
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

    let resp: PaginatedResponse<TagResource> = client.get_many("/tags", &params).await?;
    let rows: Vec<TagRow> = resp.data.iter().map(TagRow::from).collect();
    output::print_table(&rows, json);

    if let Some(links) = &resp.links {
        if links.next.is_some() && !json {
            println!("\n(more results available — use --page-size to adjust)");
        }
    }

    Ok(())
}
