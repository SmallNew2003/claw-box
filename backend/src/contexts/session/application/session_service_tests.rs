use super::session_service::{SessionError, SessionService};
use crate::contexts::runtime::adapters::ToolRegistry;
use crate::contexts::runtime::application::{AgentRuntimeService, RequestedTool};
use crate::contexts::runtime::domain::RuntimeExecutionStatus;
use serde_json::json;

fn make_service() -> SessionService {
    let mut tools = ToolRegistry::new();
    tools.register("echo", |input| Ok(input.clone()));
    SessionService::new(AgentRuntimeService::new(Box::new(tools)))
}

#[test]
fn create_and_list_sessions() {
    let mut service = make_service();

    let session = service
        .create_session("test", "2026-03-12T09:00:00Z")
        .expect("create session");
    let list = service.list_sessions();

    assert_eq!(session.session_id, "sess-1");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].status, RuntimeExecutionStatus::Idle);
}

#[test]
fn execute_message_updates_status_and_events() {
    let mut service = make_service();
    let session = service
        .create_session("test", "2026-03-12T09:00:00Z")
        .expect("create session");

    let result = service
        .execute_message(
            &session.session_id,
            "hello",
            "2026-03-12T09:00:01Z",
            Some(RequestedTool {
                name: "echo".to_string(),
                input: json!({"text": "hello"}),
            }),
        )
        .expect("execute message");
    let events = service
        .list_events(&session.session_id)
        .expect("events should exist");

    assert_eq!(result.session.status, RuntimeExecutionStatus::Completed);
    assert!(!events.is_empty());
    assert_eq!(result.tool_calls.len(), 1);
}

#[test]
fn execute_message_returns_not_found_for_unknown_session() {
    let mut service = make_service();

    let result = service.execute_message("missing", "hello", "2026-03-12T09:00:00Z", None);

    assert_eq!(
        result,
        Err(SessionError::SessionNotFound {
            session_id: "missing".to_string(),
        })
    );
}
