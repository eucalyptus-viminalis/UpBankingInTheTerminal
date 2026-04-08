use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PingMeta {
    pub id: String,
    #[serde(rename = "statusEmoji")]
    pub status_emoji: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PingResponse {
    pub meta: PingMeta,
}
