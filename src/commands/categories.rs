use serde::Serialize;
use tabled::Tabled;

use crate::client::UpClient;
use crate::models::categories::CategoryResource;
use crate::models::common::PaginatedResponse;
use crate::output;

#[derive(Tabled, Serialize)]
struct CategoryRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Parent")]
    parent: String,
    #[tabled(rename = "Children")]
    children: String,
}

impl From<&CategoryResource> for CategoryRow {
    fn from(c: &CategoryResource) -> Self {
        let parent = c
            .relationships
            .as_ref()
            .and_then(|r| r.parent.as_ref())
            .and_then(|p| p.data.as_ref())
            .map(|d| d.id.clone())
            .unwrap_or_else(|| "(root)".to_string());

        let children = c
            .relationships
            .as_ref()
            .and_then(|r| r.children.as_ref())
            .and_then(|ch| ch.data.as_ref())
            .map(|data| {
                data.iter()
                    .map(|d| d.id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        Self {
            id: c.id.clone(),
            name: c.attributes.name.clone(),
            parent,
            children: output::truncate(&children, 40),
        }
    }
}

pub async fn list(
    client: &UpClient,
    parent: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let mut params = Vec::new();
    if let Some(p) = parent {
        params.push(("filter[parent]".to_string(), p));
    }

    let resp: PaginatedResponse<CategoryResource> =
        client.get_many("/categories", &params).await?;
    let rows: Vec<CategoryRow> = resp.data.iter().map(CategoryRow::from).collect();
    output::print_table(&rows, json);
    Ok(())
}

pub async fn get(client: &UpClient, id: &str, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_one::<CategoryResource>(&format!("/categories/{}", id))
        .await?;
    let row = CategoryRow::from(&resp.data);
    output::print_single(&row, json);
    Ok(())
}
