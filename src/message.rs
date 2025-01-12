use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessage {
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
