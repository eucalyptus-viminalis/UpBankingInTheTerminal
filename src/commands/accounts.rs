use serde::Serialize;
use tabled::Tabled;

use crate::client::UpClient;
use crate::models::accounts::AccountResource;
use crate::models::common::PaginatedResponse;
use crate::output;

#[derive(Tabled, Serialize)]
struct AccountRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    account_type: String,
    #[tabled(rename = "Ownership")]
    ownership: String,
    #[tabled(rename = "Balance")]
    balance: String,
    #[tabled(rename = "Created")]
    created_at: String,
}

impl From<&AccountResource> for AccountRow {
    fn from(a: &AccountResource) -> Self {
        Self {
            id: a.id.clone(),
            name: a.attributes.display_name.clone(),
            account_type: a.attributes.account_type.to_string(),
            ownership: a.attributes.ownership_type.to_string(),
            balance: output::format_money(
                &a.attributes.balance.value,
                &a.attributes.balance.currency_code,
            ),
            created_at: a.attributes.created_at.clone(),
        }
    }
}

pub async fn list(
    client: &UpClient,
    account_type: Option<String>,
    ownership_type: Option<String>,
    page_size: Option<u8>,
    json: bool,
) -> anyhow::Result<()> {
    let mut params = Vec::new();
    if let Some(t) = account_type {
        params.push(("filter[accountType]".to_string(), t));
    }
    if let Some(o) = ownership_type {
        params.push(("filter[ownershipType]".to_string(), o));
    }
    if let Some(ps) = page_size {
        params.push(("page[size]".to_string(), ps.to_string()));
    }

    let resp: PaginatedResponse<AccountResource> = client.get_many("/accounts", &params).await?;
    let rows: Vec<AccountRow> = resp.data.iter().map(AccountRow::from).collect();
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
        .get_one::<AccountResource>(&format!("/accounts/{}", id))
        .await?;
    let row = AccountRow::from(&resp.data);
    output::print_single(&row, json);
    Ok(())
}
