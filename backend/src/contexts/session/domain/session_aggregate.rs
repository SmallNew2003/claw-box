use crate::contexts::runtime::domain::{RuntimeExecution, RuntimeExecutionStatus};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionDomainError {
    EmptyTitle,
    EmptyMessageContent,
    SessionAlreadyRunning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageRecord {
    pub message_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub title: String,
    pub status: RuntimeExecutionStatus,
    pub created_at: String,
    pub updated_at: String,
    pub latest_execution: Option<RuntimeExecution>,
    pub messages: Vec<MessageRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub title: String,
    pub status: RuntimeExecutionStatus,
    pub updated_at: String,
    pub latest_result_summary: Option<String>,
    pub latest_error_message: Option<String>,
}

impl Session {
    pub fn new(
        session_id: String,
        title: String,
        timestamp: String,
    ) -> Result<Self, SessionDomainError> {
        if title.trim().is_empty() {
            return Err(SessionDomainError::EmptyTitle);
        }

        Ok(Self {
            session_id,
            title,
            status: RuntimeExecutionStatus::Idle,
            created_at: timestamp.clone(),
            updated_at: timestamp,
            latest_execution: None,
            messages: Vec::new(),
        })
    }

    pub fn can_accept_new_message(&self) -> bool {
        self.status != RuntimeExecutionStatus::Running
    }

    pub fn append_user_message(
        &mut self,
        message_id: String,
        content: String,
        created_at: String,
    ) -> Result<(), SessionDomainError> {
        if content.trim().is_empty() {
            return Err(SessionDomainError::EmptyMessageContent);
        }

        self.messages.push(MessageRecord {
            message_id,
            role: "user".to_string(),
            content,
            created_at,
        });
        Ok(())
    }

    pub fn mark_running(&mut self, updated_at: String) -> Result<(), SessionDomainError> {
        if !self.can_accept_new_message() {
            return Err(SessionDomainError::SessionAlreadyRunning);
        }

        self.status = RuntimeExecutionStatus::Running;
        self.updated_at = updated_at;
        Ok(())
    }

    pub fn apply_execution_result(&mut self, execution: RuntimeExecution, updated_at: String) {
        self.status = execution.status();
        self.latest_execution = Some(execution);
        self.updated_at = updated_at;
    }

    pub fn to_summary(&self) -> SessionSummary {
        let latest_result_summary = self
            .latest_execution
            .as_ref()
            .and_then(|execution| execution.result_summary().map(str::to_owned));
        let latest_error_message = self
            .latest_execution
            .as_ref()
            .and_then(|execution| execution.error_message().map(str::to_owned));

        SessionSummary {
            session_id: self.session_id.clone(),
            title: self.title.clone(),
            status: self.status,
            updated_at: self.updated_at.clone(),
            latest_result_summary,
            latest_error_message,
        }
    }
}

impl fmt::Display for SessionDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionDomainError::EmptyTitle => write!(f, "session title cannot be empty"),
            SessionDomainError::EmptyMessageContent => {
                write!(f, "message content cannot be empty")
            }
            SessionDomainError::SessionAlreadyRunning => write!(f, "session is already running"),
        }
    }
}

impl std::error::Error for SessionDomainError {}
