use super::sessions::{CommandContext, SessionCommandHandler};
use crate::contexts::runtime::adapters::ToolRegistry;
use crate::contexts::runtime::application::AgentRuntimeService;
use crate::contexts::session::application::SessionService;

#[test]
fn command_execute_keeps_trace_id_in_data_plane_events() {
    let runtime = AgentRuntimeService::new(Box::new(ToolRegistry::new()));
    let service = SessionService::new(runtime);
    let mut handler = SessionCommandHandler::new(service);
    let context = CommandContext {
        trace_id: "trace-1".to_string(),
        timestamp: "2026-03-20T12:00:00Z".to_string(),
    };

    let session = handler
        .create_session("test", &context)
        .expect("create session");
    let result = handler
        .execute_session_message(&session.session_id, "hello", None, &context)
        .expect("execute message");

    let trace_id = result.events[0]
        .payload
        .get("trace_id")
        .and_then(|value| value.as_str());
    assert_eq!(trace_id, Some("trace-1"));
}
