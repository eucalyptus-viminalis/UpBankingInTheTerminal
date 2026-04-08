use serde::Serialize;
use tabled::Tabled;

use crate::client::UpClient;
use crate::models::common::PaginatedResponse;
use crate::models::transactions::TransactionResource;
use crate::output;

#[derive(Tabled, Serialize)]
struct TransactionRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Category")]
    category: String,
    #[tabled(rename = "Tags")]
    tags: String,
    #[tabled(rename = "Created")]
    created_at: String,
}

impl From<&TransactionResource> for TransactionRow {
    fn from(t: &TransactionResource) -> Self {
        let category = t
            .relationships
            .as_ref()
            .and_then(|r| r.category.as_ref())
            .and_then(|c| c.data.as_ref())
            .map(|d| d.id.clone())
            .unwrap_or_default();

        let tags = t
            .relationships
            .as_ref()
            .and_then(|r| r.tags.as_ref())
            .and_then(|tl| tl.data.as_ref())
            .map(|data| {
                data.iter()
                    .map(|d| d.id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        Self {
            id: output::truncate(&t.id, 16),
            status: t.attributes.status.to_string(),
            description: output::truncate(&t.attributes.description, 30),
            amount: output::format_money(
                &t.attributes.amount.value,
                &t.attributes.amount.currency_code,
            ),
            category,
            tags,
            created_at: t.attributes.created_at.clone(),
        }
    }
}

#[derive(Tabled, Serialize)]
struct TransactionDetail {
    #[tabled(rename = "Field")]
    field: String,
    #[tabled(rename = "Value")]
    value: String,
}

pub async fn list(
    client: &UpClient,
    account_id: Option<String>,
    status: Option<String>,
    since: Option<String>,
    until: Option<String>,
    category: Option<String>,
    tag: Option<String>,
    page_size: Option<u8>,
    json: bool,
) -> anyhow::Result<()> {
    let mut params = Vec::new();
    if let Some(s) = status {
        params.push(("filter[status]".to_string(), s));
    }
    if let Some(s) = since {
        params.push(("filter[since]".to_string(), s));
    }
    if let Some(u) = until {
        params.push(("filter[until]".to_string(), u));
    }
    if let Some(c) = category {
        params.push(("filter[category]".to_string(), c));
    }
    if let Some(t) = tag {
        params.push(("filter[tag]".to_string(), t));
    }
    if let Some(ps) = page_size {
        params.push(("page[size]".to_string(), ps.to_string()));
    }

    let path = match &account_id {
        Some(id) => format!("/accounts/{}/transactions", id),
        None => "/transactions".to_string(),
    };

    let resp: PaginatedResponse<TransactionResource> =
        client.get_many(&path, &params).await?;
    let rows: Vec<TransactionRow> = resp.data.iter().map(TransactionRow::from).collect();
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
        .get_one::<TransactionResource>(&format!("/transactions/{}", id))
        .await?;
    let t = &resp.data;

    if json {
        println!("{}", serde_json::to_string_pretty(t)?);
        return Ok(());
    }

    let mut details = vec![
        TransactionDetail {
            field: "ID".into(),
            value: t.id.clone(),
        },
        TransactionDetail {
            field: "Status".into(),
            value: t.attributes.status.to_string(),
        },
        TransactionDetail {
            field: "Description".into(),
            value: t.attributes.description.clone(),
        },
        TransactionDetail {
            field: "Amount".into(),
            value: output::format_money(
                &t.attributes.amount.value,
                &t.attributes.amount.currency_code,
            ),
        },
        TransactionDetail {
            field: "Created".into(),
            value: t.attributes.created_at.clone(),
        },
    ];

    if let Some(raw) = &t.attributes.raw_text {
        details.push(TransactionDetail {
            field: "Raw Text".into(),
            value: raw.clone(),
        });
    }

    if let Some(msg) = &t.attributes.message {
        details.push(TransactionDetail {
            field: "Message".into(),
            value: msg.clone(),
        });
    }

    if let Some(settled) = &t.attributes.settled_at {
        details.push(TransactionDetail {
            field: "Settled At".into(),
            value: settled.clone(),
        });
    }

    if let Some(foreign) = &t.attributes.foreign_amount {
        details.push(TransactionDetail {
            field: "Foreign Amount".into(),
            value: output::format_money(&foreign.value, &foreign.currency_code),
        });
    }

    if let Some(cpm) = &t.attributes.card_purchase_method {
        details.push(TransactionDetail {
            field: "Purchase Method".into(),
            value: cpm.method.to_string(),
        });
        if let Some(suffix) = &cpm.card_number_suffix {
            details.push(TransactionDetail {
                field: "Card Suffix".into(),
                value: suffix.clone(),
            });
        }
    }

    if let Some(roundup) = &t.attributes.round_up {
        details.push(TransactionDetail {
            field: "Round Up".into(),
            value: output::format_money(
                &roundup.amount.value,
                &roundup.amount.currency_code,
            ),
        });
    }

    if let Some(cashback) = &t.attributes.cashback {
        details.push(TransactionDetail {
            field: "Cashback".into(),
            value: format!(
                "{} ({})",
                output::format_money(&cashback.amount.value, &cashback.amount.currency_code),
                cashback.description
            ),
        });
    }

    if let Some(note) = &t.attributes.note {
        details.push(TransactionDetail {
            field: "Note".into(),
            value: note.text.clone(),
        });
    }

    if let Some(tt) = &t.attributes.transaction_type {
        details.push(TransactionDetail {
            field: "Type".into(),
            value: tt.clone(),
        });
    }

    // Relationships
    if let Some(rels) = &t.relationships {
        if let Some(cat) = &rels.category {
            if let Some(data) = &cat.data {
                details.push(TransactionDetail {
                    field: "Category".into(),
                    value: data.id.clone(),
                });
            }
        }
        if let Some(parent) = &rels.parent_category {
            if let Some(data) = &parent.data {
                details.push(TransactionDetail {
                    field: "Parent Category".into(),
                    value: data.id.clone(),
                });
            }
        }
        if let Some(tags) = &rels.tags {
            if let Some(data) = &tags.data {
                if !data.is_empty() {
                    let tag_str = data
                        .iter()
                        .map(|d| d.id.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    details.push(TransactionDetail {
                        field: "Tags".into(),
                        value: tag_str,
                    });
                }
            }
        }
        if let Some(acct) = &rels.account {
            if let Some(data) = &acct.data {
                details.push(TransactionDetail {
                    field: "Account".into(),
                    value: data.id.clone(),
                });
            }
        }
        if let Some(transfer) = &rels.transfer_account {
            if let Some(data) = &transfer.data {
                details.push(TransactionDetail {
                    field: "Transfer Account".into(),
                    value: data.id.clone(),
                });
            }
        }
    }

    output::print_table(&details, false);
    Ok(())
}
