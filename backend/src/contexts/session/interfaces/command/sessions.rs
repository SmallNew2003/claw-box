use crate::contexts::runtime::application::RequestedTool;
use crate::contexts::runtime::domain::{RuntimeEvent, ToolCallRecord};
use crate::contexts::session::application::{
    SessionApplicationFacade, SessionError, SessionService,
};
use crate::contexts::session::domain::Session;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandContext {
    pub trace_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteSessionMessageResult {
    pub session: Session,
    pub events: Vec<RuntimeEvent>,
    pub tool_calls: Vec<ToolCallRecord>,
}

pub struct SessionCommandHandler {
    facade: SessionApplicationFacade,
}

impl SessionCommandHandler {
    pub fn new(service: SessionService) -> Self {
        Self {
            facade: SessionApplicationFacade::new(service),
        }
    }

    pub fn create_session(
        &mut self,
        title: &str,
        context: &CommandContext,
    ) -> Result<Session, SessionError> {
        self.facade
            .create_session(title, context.timestamp.as_str())
    }

    pub fn execute_session_message(
        &mut self,
        session_id: &str,
        message_content: &str,
        requested_tool: Option<RequestedTool>,
        context: &CommandContext,
    ) -> Result<ExecuteSessionMessageResult, SessionError> {
        let (session, events, tool_calls) = self.facade.execute_message_with_trace(
            session_id,
            message_content,
            context.timestamp.as_str(),
            requested_tool,
            Some(context.trace_id.clone()),
        )?;

        Ok(ExecuteSessionMessageResult {
            session,
            events,
            tool_calls,
        })
    }
}
