use serde::{Deserialize, Serialize};

/// Monetary amount as returned by the Up API.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyObject {
    pub currency_code: String,
    pub value: String,
    pub value_in_base_units: i64,
}

/// Pagination links.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationLinks {
    pub prev: Option<String>,
    pub next: Option<String>,
}

/// A generic paginated response.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub links: Option<PaginationLinks>,
}

/// A single-resource response.
#[derive(Debug, Clone, Deserialize)]
pub struct SingleResponse<T> {
    pub data: T,
}

/// Relationship link.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationshipLink {
    pub related: Option<String>,
    #[serde(rename = "self")]
    pub self_link: Option<String>,
}

/// Relationship data reference.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationshipData {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
}

/// A relationship with optional data and links.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Relationship {
    pub data: Option<RelationshipData>,
    pub links: Option<RelationshipLink>,
}

/// A relationship with a list of data references.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationshipList {
    pub data: Option<Vec<RelationshipData>>,
    pub links: Option<RelationshipLink>,
}

/// Self link on a resource.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelfLink {
    #[serde(rename = "self")]
    pub self_link: Option<String>,
}

/// API error source.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorSource {
    pub parameter: Option<String>,
    pub pointer: Option<String>,
}

/// A single API error.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    pub status: String,
    pub title: String,
    pub detail: String,
    pub source: Option<ErrorSource>,
}

/// API error response envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<ApiError>,
}
