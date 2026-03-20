use super::agent_runtime_service::{AgentRuntimeService, ExecutionInput, RequestedTool};
use crate::contexts::runtime::adapters::{InMemoryRuntimeStores, ToolRegistry};
use crate::contexts::runtime::domain::{RuntimeEventType, RuntimeExecutionStatus};
use crate::contexts::runtime::ports::{ToolInvocationError, ToolInvokerPort};
use serde_json::json;

struct FakeToolInvoker;

impl ToolInvokerPort for FakeToolInvoker {
    fn invoke(
        &self,
        tool_name: &str,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, ToolInvocationError> {
        if tool_name == "fake" {
            Ok(json!({"echo": input}))
        } else {
            Err(ToolInvocationError::NotFound {
                tool_name: tool_name.to_string(),
            })
        }
    }
}

#[test]
fn execute_without_tool_produces_completed_execution() {
    let runtime = AgentRuntimeService::new(Box::new(ToolRegistry::new()));
    let input = ExecutionInput {
        execution_id: "exec-1".to_string(),
        session_id: "sess-1".to_string(),
        message_id: "msg-1".to_string(),
        trace_id: None,
        message_content: "hello".to_string(),
        started_at: "2026-03-12T09:00:00Z".to_string(),
        completed_at: "2026-03-12T09:00:01Z".to_string(),
        requested_tool: None,
    };

    let report = runtime.execute(input).expect("execute should succeed");

    assert_eq!(report.execution.status(), RuntimeExecutionStatus::Completed);
    assert_eq!(report.tool_calls.len(), 0);
    assert!(report
        .events
        .iter()
        .any(|event| event.event_type == RuntimeEventType::ExecutionCompleted));
}

#[test]
fn execute_with_tool_records_tool_call() {
    let mut tools = ToolRegistry::new();
    tools.register("uppercase", |input| {
        let text = input
            .get("text")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_uppercase();
        Ok(json!({"text": text}))
    });

    let runtime = AgentRuntimeService::new(Box::new(tools));
    let input = ExecutionInput {
        execution_id: "exec-1".to_string(),
        session_id: "sess-1".to_string(),
        message_id: "msg-1".to_string(),
        trace_id: None,
        message_content: "call tool".to_string(),
        started_at: "2026-03-12T09:00:00Z".to_string(),
        completed_at: "2026-03-12T09:00:01Z".to_string(),
        requested_tool: Some(RequestedTool {
            name: "uppercase".to_string(),
            input: json!({"text": "hello"}),
        }),
    };

    let report = runtime.execute(input).expect("execute should succeed");

    assert_eq!(report.execution.status(), RuntimeExecutionStatus::Completed);
    assert_eq!(report.tool_calls.len(), 1);
    assert_eq!(
        report.tool_calls[0].status,
        RuntimeExecutionStatus::Completed
    );
}

#[test]
fn execute_with_missing_tool_marks_execution_failed() {
    let runtime = AgentRuntimeService::new(Box::new(ToolRegistry::new()));
    let input = ExecutionInput {
        execution_id: "exec-1".to_string(),
        session_id: "sess-1".to_string(),
        message_id: "msg-1".to_string(),
        trace_id: None,
        message_content: "call missing tool".to_string(),
        started_at: "2026-03-12T09:00:00Z".to_string(),
        completed_at: "2026-03-12T09:00:01Z".to_string(),
        requested_tool: Some(RequestedTool {
            name: "missing".to_string(),
            input: json!({}),
        }),
    };

    let report = runtime
        .execute(input)
        .expect("execution report should be returned");

    assert_eq!(report.execution.status(), RuntimeExecutionStatus::Failed);
    assert_eq!(report.tool_calls.len(), 1);
    assert!(report
        .events
        .iter()
        .any(|event| event.event_type == RuntimeEventType::ExecutionFailed));
}

#[test]
fn execute_with_adapters_persists_side_effects() {
    let runtime = AgentRuntimeService::new(Box::new(ToolRegistry::new()));
    let input = ExecutionInput {
        execution_id: "exec-1".to_string(),
        session_id: "sess-1".to_string(),
        message_id: "msg-1".to_string(),
        trace_id: None,
        message_content: "hello".to_string(),
        started_at: "2026-03-12T09:00:00Z".to_string(),
        completed_at: "2026-03-12T09:00:01Z".to_string(),
        requested_tool: None,
    };
    let mut stores = InMemoryRuntimeStores::default();

    let report = runtime
        .execute_with_adapters(input, &mut stores)
        .expect("execute should succeed");

    assert_eq!(stores.events.len(), report.events.len());
}

#[test]
fn execute_with_trace_id_puts_trace_id_into_event_payload() {
    let runtime = AgentRuntimeService::new(Box::new(ToolRegistry::new()));
    let input = ExecutionInput {
        execution_id: "exec-1".to_string(),
        session_id: "sess-1".to_string(),
        message_id: "msg-1".to_string(),
        trace_id: Some("trace-1".to_string()),
        message_content: "hello".to_string(),
        started_at: "2026-03-12T09:00:00Z".to_string(),
        completed_at: "2026-03-12T09:00:01Z".to_string(),
        requested_tool: None,
    };

    let report = runtime.execute(input).expect("execute should succeed");
    let trace_id = report.events[0]
        .payload
        .get("trace_id")
        .and_then(|value| value.as_str());

    assert_eq!(trace_id, Some("trace-1"));
}

#[test]
fn execute_accepts_any_tool_invoker_port_impl() {
    let runtime = AgentRuntimeService::new(Box::new(FakeToolInvoker));
    let input = ExecutionInput {
        execution_id: "exec-1".to_string(),
        session_id: "sess-1".to_string(),
        message_id: "msg-1".to_string(),
        trace_id: None,
        message_content: "hello".to_string(),
        started_at: "2026-03-12T09:00:00Z".to_string(),
        completed_at: "2026-03-12T09:00:01Z".to_string(),
        requested_tool: Some(RequestedTool {
            name: "fake".to_string(),
            input: json!({"text": "hello"}),
        }),
    };

    let report = runtime.execute(input).expect("execute should succeed");

    assert_eq!(report.execution.status(), RuntimeExecutionStatus::Completed);
    assert_eq!(report.tool_calls.len(), 1);
}
