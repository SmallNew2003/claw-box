use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeExecutionStatus {
    Idle,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub latency_ms: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeExecution {
    execution_id: String,
    session_id: String,
    input_message_id: String,
    status: RuntimeExecutionStatus,
    started_at: Option<String>,
    completed_at: Option<String>,
    result_summary: Option<String>,
    error_message: Option<String>,
    metrics: Option<RuntimeMetrics>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStateError {
    pub from: RuntimeExecutionStatus,
    pub to: RuntimeExecutionStatus,
}

impl RuntimeExecution {
    pub fn new(
        execution_id: impl Into<String>,
        session_id: impl Into<String>,
        input_message_id: impl Into<String>,
    ) -> Self {
        Self {
            execution_id: execution_id.into(),
            session_id: session_id.into(),
            input_message_id: input_message_id.into(),
            status: RuntimeExecutionStatus::Idle,
            started_at: None,
            completed_at: None,
            result_summary: None,
            error_message: None,
            metrics: None,
        }
    }

    pub fn start(&mut self, started_at: impl Into<String>) -> Result<(), RuntimeStateError> {
        if self.status != RuntimeExecutionStatus::Idle {
            return Err(RuntimeStateError {
                from: self.status,
                to: RuntimeExecutionStatus::Running,
            });
        }

        self.status = RuntimeExecutionStatus::Running;
        self.started_at = Some(started_at.into());
        Ok(())
    }

    pub fn complete(
        &mut self,
        completed_at: impl Into<String>,
        result_summary: Option<String>,
        metrics: Option<RuntimeMetrics>,
    ) -> Result<(), RuntimeStateError> {
        if self.status != RuntimeExecutionStatus::Running {
            return Err(RuntimeStateError {
                from: self.status,
                to: RuntimeExecutionStatus::Completed,
            });
        }

        self.status = RuntimeExecutionStatus::Completed;
        self.completed_at = Some(completed_at.into());
        self.result_summary = result_summary;
        self.metrics = metrics;
        self.error_message = None;
        Ok(())
    }

    pub fn fail(
        &mut self,
        completed_at: impl Into<String>,
        error_message: impl Into<String>,
        metrics: Option<RuntimeMetrics>,
    ) -> Result<(), RuntimeStateError> {
        if self.status != RuntimeExecutionStatus::Running {
            return Err(RuntimeStateError {
                from: self.status,
                to: RuntimeExecutionStatus::Failed,
            });
        }

        self.status = RuntimeExecutionStatus::Failed;
        self.completed_at = Some(completed_at.into());
        self.error_message = Some(error_message.into());
        self.metrics = metrics;
        self.result_summary = None;
        Ok(())
    }

    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn input_message_id(&self) -> &str {
        &self.input_message_id
    }

    pub fn status(&self) -> RuntimeExecutionStatus {
        self.status
    }

    pub fn started_at(&self) -> Option<&str> {
        self.started_at.as_deref()
    }

    pub fn completed_at(&self) -> Option<&str> {
        self.completed_at.as_deref()
    }

    pub fn result_summary(&self) -> Option<&str> {
        self.result_summary.as_deref()
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn metrics(&self) -> Option<&RuntimeMetrics> {
        self.metrics.as_ref()
    }
}

impl fmt::Display for RuntimeStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid runtime execution transition from {} to {}",
            status_label(self.from),
            status_label(self.to)
        )
    }
}

impl std::error::Error for RuntimeStateError {}

fn status_label(status: RuntimeExecutionStatus) -> &'static str {
    match status {
        RuntimeExecutionStatus::Idle => "idle",
        RuntimeExecutionStatus::Running => "running",
        RuntimeExecutionStatus::Completed => "completed",
        RuntimeExecutionStatus::Failed => "failed",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeEventType {
    ExecutionStarted,
    StepProgress,
    ToolCallStarted,
    ToolCallCompleted,
    ExecutionCompleted,
    ExecutionFailed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub event_id: String,
    pub execution_id: String,
    pub session_id: String,
    #[serde(rename = "type")]
    pub event_type: RuntimeEventType,
    pub payload: serde_json::Value,
    pub created_at: String,
}
