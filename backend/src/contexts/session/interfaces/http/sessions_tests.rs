use super::sessions::SessionHttpHandler;
use crate::contexts::runtime::adapters::ToolRegistry;
use crate::contexts::runtime::application::AgentRuntimeService;
use crate::contexts::session::application::SessionService;

#[test]
fn stream_event_history_sse_contains_trace_id_in_data() {
    let runtime = AgentRuntimeService::new(Box::new(ToolRegistry::new()));
    let mut service = SessionService::new(runtime);

    let created = service
        .create_session("test", "2026-03-20T12:00:00Z")
        .expect("create session");
    service
        .execute_message_with_trace(
            &created.session_id,
            "hello",
            "2026-03-20T12:00:01Z",
            None,
            Some("trace-1".to_string()),
        )
        .expect("execute message");

    let handler = SessionHttpHandler::new(service);
    let sse = handler
        .stream_event_history_sse(&created.session_id, "2026-03-20T12:00:02Z", "trace-2")
        .expect("stream history");

    assert!(sse.contains("event: execution_started"));
    assert!(sse.contains("\"trace_id\":\"trace-1\""));
}
