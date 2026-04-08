use serde::{Deserialize, Serialize};

use super::common::{Relationship, SelfLink};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookAttributes {
    pub url: String,
    pub description: Option<String>,
    pub secret_key: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookRelationships {
    pub logs: Option<Relationship>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: WebhookAttributes,
    pub relationships: Option<WebhookRelationships>,
    pub links: Option<SelfLink>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookLogAttributes {
    pub status_code: Option<u16>,
    pub body: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookLogRelationships {
    pub webhook: Option<Relationship>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookLogResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: WebhookLogAttributes,
    pub relationships: Option<WebhookLogRelationships>,
    pub links: Option<SelfLink>,
}
