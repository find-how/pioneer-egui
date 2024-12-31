use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordedEvent {
    pub event_type: String,
    pub component_id: String,
    pub event_data: Value,
    pub timestamp: u64,
}
