use serde::{Deserialize, Serialize};

use super::common::{Relationship, SelfLink};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentAttributes {
    pub created_at: Option<String>,
    pub file_url: Option<String>,
    pub file_url_expires_at: Option<String>,
    pub file_extension: Option<String>,
    pub file_content_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentRelationships {
    pub transaction: Option<Relationship>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttachmentResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: AttachmentAttributes,
    pub relationships: Option<AttachmentRelationships>,
    pub links: Option<SelfLink>,
}
