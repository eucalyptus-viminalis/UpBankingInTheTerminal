use serde::Deserialize;

use super::common::{Relationship, RelationshipList, SelfLink};

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryAttributes {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryRelationships {
    pub parent: Option<Relationship>,
    pub children: Option<RelationshipList>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: CategoryAttributes,
    pub relationships: Option<CategoryRelationships>,
    pub links: Option<SelfLink>,
}
