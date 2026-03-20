use super::{SessionError, SessionService};
use crate::contexts::runtime::application::RequestedTool;
use crate::contexts::runtime::domain::{RuntimeEvent, ToolCallRecord};
use crate::contexts::session::domain::{Session, SessionSummary};

/// Session 上下文应用门面。
///
/// 所有接口入口都通过该门面调用应用服务，避免跨层直接访问。
pub struct SessionApplicationFacade {
    service: SessionService,
}

impl SessionApplicationFacade {
    pub fn new(service: SessionService) -> Self {
        Self { service }
    }

    pub fn create_session(
        &mut self,
        title: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Result<Session, SessionError> {
        self.service.create_session(title, timestamp)
    }

    pub fn list_sessions(&self) -> Vec<SessionSummary> {
        self.service.list_sessions()
    }

    pub fn get_session(&self, session_id: &str) -> Result<Session, SessionError> {
        self.service.get_session(session_id)
    }

    pub fn execute_message(
        &mut self,
        session_id: &str,
        message_content: impl Into<String>,
        timestamp: impl Into<String>,
        requested_tool: Option<RequestedTool>,
    ) -> Result<(Session, Vec<RuntimeEvent>, Vec<ToolCallRecord>), SessionError> {
        self.execute_message_with_trace(
            session_id,
            message_content,
            timestamp,
            requested_tool,
            None,
        )
    }

    pub fn execute_message_with_trace(
        &mut self,
        session_id: &str,
        message_content: impl Into<String>,
        timestamp: impl Into<String>,
        requested_tool: Option<RequestedTool>,
        trace_id: Option<String>,
    ) -> Result<(Session, Vec<RuntimeEvent>, Vec<ToolCallRecord>), SessionError> {
        let result = self.service.execute_message_with_trace(
            session_id,
            message_content,
            timestamp,
            requested_tool,
            trace_id,
        )?;
        Ok((result.session, result.events, result.tool_calls))
    }

    pub fn list_events(&self, session_id: &str) -> Result<Vec<RuntimeEvent>, SessionError> {
        self.service.list_events(session_id)
    }

    pub fn into_inner(self) -> SessionService {
        self.service
    }
}
