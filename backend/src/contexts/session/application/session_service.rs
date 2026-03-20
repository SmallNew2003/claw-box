use crate::contexts::runtime::adapters::InMemoryRuntimeStores;
use crate::contexts::runtime::application::{
    AgentRuntimeError, AgentRuntimeService, ExecutionInput, RequestedTool,
};
use crate::contexts::runtime::domain::{RuntimeEvent, ToolCallRecord};
use crate::contexts::session::adapters::{InMemorySessionEventStore, InMemorySessionRepository};
use crate::contexts::session::domain::{Session, SessionDomainError, SessionSummary};
use crate::contexts::session::ports::{SessionEventStorePort, SessionRepositoryPort};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// 会话服务层错误，映射到 API 错误码时保持语义稳定。
pub enum SessionError {
    SessionNotFound { session_id: String },
    SessionRunning { session_id: String },
    ValidationError { message: String },
    RuntimeFailure { message: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// 一次消息执行后的会话视图，返回给 API 层作为统一响应体。
pub struct SessionExecutionResult {
    pub session: Session,
    pub events: Vec<RuntimeEvent>,
    pub tool_calls: Vec<ToolCallRecord>,
}

pub struct SessionService {
    runtime: AgentRuntimeService,
    runtime_stores: InMemoryRuntimeStores,
    sessions: Box<dyn SessionRepositoryPort>,
    event_store: Box<dyn SessionEventStorePort>,
    next_session_seq: u64,
    next_message_seq: u64,
    next_execution_seq: u64,
}

impl SessionService {
    /// 创建默认内存适配器组合的会话服务。
    /// 创建会话服务实例。
    pub fn new(runtime: AgentRuntimeService) -> Self {
        Self::with_ports(
            runtime,
            Box::new(InMemorySessionRepository::default()),
            Box::new(InMemorySessionEventStore::default()),
        )
    }

    /// 通过端口注入构造会话服务，应用层仅依赖端口抽象。
    pub fn with_ports(
        runtime: AgentRuntimeService,
        sessions: Box<dyn SessionRepositoryPort>,
        event_store: Box<dyn SessionEventStorePort>,
    ) -> Self {
        Self {
            runtime,
            runtime_stores: InMemoryRuntimeStores::default(),
            sessions,
            event_store,
            next_session_seq: 1,
            next_message_seq: 1,
            next_execution_seq: 1,
        }
    }

    /// 创建新会话，初始化状态为 idle。
    pub fn create_session(
        &mut self,
        title: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Result<Session, SessionError> {
        let session_id = format!("sess-{}", self.next_session_seq);
        self.next_session_seq += 1;
        let now = timestamp.into();
        let session = Session::new(session_id, title.into(), now).map_err(|error| {
            SessionError::ValidationError {
                message: error.to_string(),
            }
        })?;

        self.sessions.save(session.clone());
        Ok(session)
    }

    /// 获取会话摘要列表，用于会话列表页展示。
    pub fn list_sessions(&self) -> Vec<SessionSummary> {
        let mut summaries = self
            .sessions
            .list()
            .into_iter()
            .map(|session| session.to_summary())
            .collect::<Vec<_>>();
        summaries.sort_by(|left, right| left.session_id.cmp(&right.session_id));
        summaries
    }

    /// 获取会话详情。
    pub fn get_session(&self, session_id: &str) -> Result<Session, SessionError> {
        self.sessions
            .find_by_id(session_id)
            .ok_or_else(|| SessionError::SessionNotFound {
                session_id: session_id.to_string(),
            })
    }

    /// 在指定会话中执行一次消息。
    ///
    /// 约束：会话处于 running 时拒绝重复触发。
    pub fn execute_message(
        &mut self,
        session_id: &str,
        message_content: impl Into<String>,
        timestamp: impl Into<String>,
        requested_tool: Option<RequestedTool>,
    ) -> Result<SessionExecutionResult, SessionError> {
        self.execute_message_with_trace(
            session_id,
            message_content,
            timestamp,
            requested_tool,
            None,
        )
    }

    /// 在指定会话中执行一次消息，并附带端到端 trace_id。
    pub fn execute_message_with_trace(
        &mut self,
        session_id: &str,
        message_content: impl Into<String>,
        timestamp: impl Into<String>,
        requested_tool: Option<RequestedTool>,
        trace_id: Option<String>,
    ) -> Result<SessionExecutionResult, SessionError> {
        let message_content = message_content.into();

        let timestamp = timestamp.into();
        let mut session = self.get_session(session_id)?;

        let message_id = format!("msg-{}", self.next_message_seq);
        self.next_message_seq += 1;
        session
            .append_user_message(
                message_id.clone(),
                message_content.clone(),
                timestamp.clone(),
            )
            .map_err(|error| map_domain_error_for_session(error, session_id))?;

        session
            .mark_running(timestamp.clone())
            .map_err(|error| map_domain_error_for_session(error, session_id))?;
        let execution_id = format!("exec-{}", self.next_execution_seq);
        self.next_execution_seq += 1;
        let execution_input = ExecutionInput {
            execution_id,
            session_id: session_id.to_string(),
            message_id,
            trace_id,
            message_content,
            started_at: timestamp.clone(),
            completed_at: timestamp.clone(),
            requested_tool,
        };

        let report = self
            .runtime
            .execute_with_adapters(execution_input, &mut self.runtime_stores)
            .map_err(map_runtime_error)?;
        session.apply_execution_result(report.execution.clone(), timestamp);
        self.sessions.save(session.clone());
        self.event_store.append(session_id, &report.events);

        Ok(SessionExecutionResult {
            session,
            events: report.events,
            tool_calls: report.tool_calls,
        })
    }

    /// 查询会话历史事件。
    pub fn list_events(&self, session_id: &str) -> Result<Vec<RuntimeEvent>, SessionError> {
        self.get_session(session_id)?;
        Ok(self.event_store.list_by_session(session_id))
    }
}

fn map_runtime_error(error: AgentRuntimeError) -> SessionError {
    SessionError::RuntimeFailure {
        message: error.to_string(),
    }
}

fn map_domain_error_for_session(error: SessionDomainError, session_id: &str) -> SessionError {
    match error {
        SessionDomainError::SessionAlreadyRunning => SessionError::SessionRunning {
            session_id: session_id.to_string(),
        },
        SessionDomainError::EmptyTitle | SessionDomainError::EmptyMessageContent => {
            SessionError::ValidationError {
                message: error.to_string(),
            }
        }
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionError::SessionNotFound { session_id } => {
                write!(f, "session '{}' not found", session_id)
            }
            SessionError::SessionRunning { session_id } => {
                write!(f, "session '{}' is already running", session_id)
            }
            SessionError::ValidationError { message } => write!(f, "validation error: {}", message),
            SessionError::RuntimeFailure { message } => write!(f, "runtime failure: {}", message),
        }
    }
}

impl std::error::Error for SessionError {}
