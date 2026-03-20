use super::sessions::{ApiError, SessionsApi};
use crate::contexts::runtime::adapters::ToolRegistry;
use crate::contexts::runtime::application::AgentRuntimeService;
use crate::contexts::session::application::SessionService;

fn make_api() -> SessionsApi {
    let runtime = AgentRuntimeService::new(Box::new(ToolRegistry::new()));
    let service = SessionService::new(runtime);
    SessionsApi::new(service)
}

#[test]
fn execute_message_on_missing_session_returns_session_not_found() {
    let mut api = make_api();
    let result = api.execute_message("missing", "hello", None, "2026-03-12T09:00:00Z", "req-1");

    assert_eq!(
        result,
        Err(ApiError {
            code: "SESSION_NOT_FOUND",
            message: "session 'missing' not found".to_string(),
        })
    );
}

#[test]
fn create_execute_and_query_event_history_keeps_main_flow_stable() {
    let mut api = make_api();
    let created = api
        .create_session("test", "2026-03-12T09:00:00Z", "req-1")
        .expect("create session");

    let executed = api
        .execute_message(
            &created.data.session_id,
            "hello",
            None,
            "2026-03-12T09:00:01Z",
            "req-1",
        )
        .expect("execute message");
    let history = api
        .list_event_history(&created.data.session_id, "2026-03-12T09:00:02Z", "req-2")
        .expect("list history");

    assert_eq!(executed.data.session.session_id, created.data.session_id);
    assert!(!history.data.is_empty());
    let trace_id = history.data[0]
        .payload
        .get("trace_id")
        .and_then(|value| value.as_str());
    assert_eq!(trace_id, Some("req-1"));
}
