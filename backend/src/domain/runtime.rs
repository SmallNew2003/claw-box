use serde::{Deserialize, Serialize};

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
    pub execution_id: String,
    pub session_id: String,
    pub input_message_id: String,
    pub status: RuntimeExecutionStatus,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub result_summary: Option<String>,
    pub error_message: Option<String>,
    pub metrics: Option<RuntimeMetrics>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStateError {
    pub from: RuntimeExecutionStatus,
    pub to: RuntimeExecutionStatus,
}

impl RuntimeExecution {
    pub fn new(execution_id: impl Into<String>, session_id: impl Into<String>, input_message_id: impl Into<String>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_starts_in_idle_state() {
        let execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");

        assert_eq!(execution.status, RuntimeExecutionStatus::Idle);
        assert_eq!(execution.started_at, None);
        assert_eq!(execution.completed_at, None);
        assert_eq!(execution.result_summary, None);
        assert_eq!(execution.error_message, None);
    }

    #[test]
    fn start_transitions_idle_to_running_and_sets_started_at() {
        let mut execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");

        let result = execution.start("2026-03-12T09:00:00Z");

        assert!(result.is_ok());
        assert_eq!(execution.status, RuntimeExecutionStatus::Running);
        assert_eq!(execution.started_at.as_deref(), Some("2026-03-12T09:00:00Z"));
    }

    #[test]
    fn start_from_running_returns_error() {
        let mut execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");
        let _ = execution.start("2026-03-12T09:00:00Z");

        let result = execution.start("2026-03-12T09:01:00Z");

        assert_eq!(
            result,
            Err(RuntimeStateError {
                from: RuntimeExecutionStatus::Running,
                to: RuntimeExecutionStatus::Running,
            })
        );
    }

    #[test]
    fn complete_from_running_sets_completed_at_and_metrics() {
        let mut execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");
        let _ = execution.start("2026-03-12T09:00:00Z");
        let metrics = RuntimeMetrics {
            latency_ms: 120,
            input_tokens: 12,
            output_tokens: 34,
        };

        let result = execution.complete(
            "2026-03-12T09:00:02Z",
            Some("done".to_string()),
            Some(metrics.clone()),
        );

        assert!(result.is_ok());
        assert_eq!(execution.status, RuntimeExecutionStatus::Completed);
        assert_eq!(execution.completed_at.as_deref(), Some("2026-03-12T09:00:02Z"));
        assert_eq!(execution.result_summary.as_deref(), Some("done"));
        assert_eq!(execution.metrics, Some(metrics));
    }

    #[test]
    fn complete_from_idle_returns_error() {
        let mut execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");

        let result = execution.complete("2026-03-12T09:00:02Z", None, None);

        assert_eq!(
            result,
            Err(RuntimeStateError {
                from: RuntimeExecutionStatus::Idle,
                to: RuntimeExecutionStatus::Completed,
            })
        );
    }

    #[test]
    fn fail_from_running_sets_error_and_completed_at() {
        let mut execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");
        let _ = execution.start("2026-03-12T09:00:00Z");
        let metrics = RuntimeMetrics {
            latency_ms: 80,
            input_tokens: 10,
            output_tokens: 0,
        };

        let result = execution.fail(
            "2026-03-12T09:00:01Z",
            "timeout",
            Some(metrics.clone()),
        );

        assert!(result.is_ok());
        assert_eq!(execution.status, RuntimeExecutionStatus::Failed);
        assert_eq!(execution.completed_at.as_deref(), Some("2026-03-12T09:00:01Z"));
        assert_eq!(execution.error_message.as_deref(), Some("timeout"));
        assert_eq!(execution.metrics, Some(metrics));
    }

    #[test]
    fn runtime_event_type_serializes_to_snake_case() {
        let event_type = RuntimeEventType::ExecutionStarted;

        let json = serde_json::to_string(&event_type).expect("serialize event type");

        assert_eq!(json, "\"execution_started\"");
    }
}
