use crate::contexts::runtime::domain::{RuntimeEvent, RuntimeEventType};
use crate::contexts::session::application::SessionService;
use crate::contexts::session::domain::SessionSummary;
use crate::contexts::session::interfaces::api::sessions::{ApiError, ApiResponse, SessionsApi};

pub struct SessionHttpHandler {
    api: SessionsApi,
}

impl SessionHttpHandler {
    pub fn new(service: SessionService) -> Self {
        Self {
            api: SessionsApi::new(service),
        }
    }

    pub fn list_sessions(
        &self,
        timestamp: &str,
        request_id: &str,
    ) -> Result<ApiResponse<Vec<SessionSummary>>, ApiError> {
        self.api.list_sessions(timestamp, request_id)
    }

    pub fn stream_event_history_sse(
        &self,
        session_id: &str,
        timestamp: &str,
        request_id: &str,
    ) -> Result<String, ApiError> {
        let response = self
            .api
            .list_event_history(session_id, timestamp, request_id)?;
        let mut payload = String::new();
        for event in response.data {
            payload.push_str(&to_sse_frame(&event));
        }
        Ok(payload)
    }
}

fn to_sse_frame(event: &RuntimeEvent) -> String {
    let data = serde_json::to_string(event).expect("runtime event should serialize");
    format!(
        "id: {}\nevent: {}\ndata: {}\n\n",
        event.event_id,
        event_type_name(event.event_type),
        data
    )
}

fn event_type_name(event_type: RuntimeEventType) -> &'static str {
    match event_type {
        RuntimeEventType::ExecutionStarted => "execution_started",
        RuntimeEventType::StepProgress => "step_progress",
        RuntimeEventType::ToolCallStarted => "tool_call_started",
        RuntimeEventType::ToolCallCompleted => "tool_call_completed",
        RuntimeEventType::ExecutionCompleted => "execution_completed",
        RuntimeEventType::ExecutionFailed => "execution_failed",
    }
}
