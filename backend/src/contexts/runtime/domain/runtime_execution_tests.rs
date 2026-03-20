use super::runtime_execution::{
    RuntimeEventType, RuntimeExecution, RuntimeExecutionStatus, RuntimeMetrics, RuntimeStateError,
};

#[test]
fn execution_starts_in_idle_state() {
    let execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");

    assert_eq!(execution.status(), RuntimeExecutionStatus::Idle);
    assert_eq!(execution.started_at(), None);
    assert_eq!(execution.completed_at(), None);
    assert_eq!(execution.result_summary(), None);
    assert_eq!(execution.error_message(), None);
}

#[test]
fn start_transitions_idle_to_running_and_sets_started_at() {
    let mut execution = RuntimeExecution::new("exec-1", "sess-1", "msg-1");

    let result = execution.start("2026-03-12T09:00:00Z");

    assert!(result.is_ok());
    assert_eq!(execution.status(), RuntimeExecutionStatus::Running);
    assert_eq!(execution.started_at(), Some("2026-03-12T09:00:00Z"));
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
    assert_eq!(execution.status(), RuntimeExecutionStatus::Completed);
    assert_eq!(execution.completed_at(), Some("2026-03-12T09:00:02Z"));
    assert_eq!(execution.result_summary(), Some("done"));
    assert_eq!(execution.metrics(), Some(&metrics));
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

    let result = execution.fail("2026-03-12T09:00:01Z", "timeout", Some(metrics.clone()));

    assert!(result.is_ok());
    assert_eq!(execution.status(), RuntimeExecutionStatus::Failed);
    assert_eq!(execution.completed_at(), Some("2026-03-12T09:00:01Z"));
    assert_eq!(execution.error_message(), Some("timeout"));
    assert_eq!(execution.metrics(), Some(&metrics));
}

#[test]
fn runtime_state_error_formats_transition() {
    let error = RuntimeStateError {
        from: RuntimeExecutionStatus::Idle,
        to: RuntimeExecutionStatus::Running,
    };

    assert_eq!(
        error.to_string(),
        "invalid runtime execution transition from idle to running"
    );
}

#[test]
fn runtime_event_type_serializes_to_snake_case() {
    let event_type = RuntimeEventType::ExecutionStarted;

    let json = serde_json::to_string(&event_type).expect("serialize event type");

    assert_eq!(json, "\"execution_started\"");
}
