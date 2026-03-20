use crate::contexts::runtime::application::RequestedTool;
use crate::contexts::runtime::domain::{RuntimeEvent, ToolCallRecord};
use crate::contexts::session::application::{
    SessionApplicationFacade, SessionError, SessionService,
};
use crate::contexts::session::domain::{Session, SessionSummary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiMeta {
    pub timestamp: String,
    pub request_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: ApiMeta,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteMessageResponse {
    pub session: Session,
    pub events: Vec<RuntimeEvent>,
    pub tool_calls: Vec<ToolCallRecord>,
}

pub struct SessionsApi {
    /// 统一通过 application facade 进入用例编排，避免接口层直接触达领域细节。
    pub facade: SessionApplicationFacade,
}

impl SessionsApi {
    pub fn new(service: SessionService) -> Self {
        Self {
            facade: SessionApplicationFacade::new(service),
        }
    }

    pub fn create_session(
        &mut self,
        title: &str,
        timestamp: &str,
        request_id: &str,
    ) -> Result<ApiResponse<Session>, ApiError> {
        let session = self
            .facade
            .create_session(title, timestamp)
            .map_err(map_api_error)?;
        Ok(ApiResponse {
            data: session,
            meta: api_meta(timestamp, request_id),
        })
    }

    pub fn list_sessions(
        &self,
        timestamp: &str,
        request_id: &str,
    ) -> Result<ApiResponse<Vec<SessionSummary>>, ApiError> {
        Ok(ApiResponse {
            data: self.facade.list_sessions(),
            meta: api_meta(timestamp, request_id),
        })
    }

    pub fn get_session(
        &self,
        session_id: &str,
        timestamp: &str,
        request_id: &str,
    ) -> Result<ApiResponse<Session>, ApiError> {
        let session = self.facade.get_session(session_id).map_err(map_api_error)?;
        Ok(ApiResponse {
            data: session,
            meta: api_meta(timestamp, request_id),
        })
    }

    pub fn execute_message(
        &mut self,
        session_id: &str,
        message_content: &str,
        requested_tool: Option<RequestedTool>,
        timestamp: &str,
        request_id: &str,
    ) -> Result<ApiResponse<ExecuteMessageResponse>, ApiError> {
        let (session, events, tool_calls) = self
            .facade
            .execute_message_with_trace(
                session_id,
                message_content,
                timestamp,
                requested_tool,
                Some(request_id.to_string()),
            )
            .map_err(map_api_error)?;

        Ok(ApiResponse {
            data: ExecuteMessageResponse {
                session,
                events,
                tool_calls,
            },
            meta: api_meta(timestamp, request_id),
        })
    }

    pub fn list_event_history(
        &self,
        session_id: &str,
        timestamp: &str,
        request_id: &str,
    ) -> Result<ApiResponse<Vec<RuntimeEvent>>, ApiError> {
        let events = self.facade.list_events(session_id).map_err(map_api_error)?;

        Ok(ApiResponse {
            data: events,
            meta: api_meta(timestamp, request_id),
        })
    }
}

fn api_meta(timestamp: &str, request_id: &str) -> ApiMeta {
    ApiMeta {
        timestamp: timestamp.to_string(),
        request_id: request_id.to_string(),
    }
}

fn map_api_error(error: SessionError) -> ApiError {
    match error {
        SessionError::SessionNotFound { .. } => ApiError {
            code: "SESSION_NOT_FOUND",
            message: error.to_string(),
        },
        SessionError::SessionRunning { .. } => ApiError {
            code: "SESSION_RUNNING",
            message: error.to_string(),
        },
        SessionError::ValidationError { .. } => ApiError {
            code: "VALIDATION_ERROR",
            message: error.to_string(),
        },
        SessionError::RuntimeFailure { .. } => ApiError {
            code: "INTERNAL_ERROR",
            message: error.to_string(),
        },
    }
}
