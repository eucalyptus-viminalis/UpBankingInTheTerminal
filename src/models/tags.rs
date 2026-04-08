use serde::Deserialize;

use super::common::RelationshipList;

#[derive(Debug, Clone, Deserialize)]
pub struct TagResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub relationships: Option<TagRelationships>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagRelationships {
    pub transactions: Option<RelationshipList>,
}
