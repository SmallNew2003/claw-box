use super::RuntimeExecutionStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallRecord {
    pub tool_call_id: String,
    pub execution_id: String,
    pub session_id: String,
    pub tool_name: String,
    pub status: RuntimeExecutionStatus,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub started_at: String,
    pub completed_at: Option<String>,
}
