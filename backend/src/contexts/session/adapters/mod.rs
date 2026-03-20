use crate::contexts::runtime::domain::RuntimeEvent;
use crate::contexts::session::domain::Session;
use crate::contexts::session::ports::{SessionEventStorePort, SessionRepositoryPort};
use std::collections::HashMap;

/// 过渡期内存会话仓储。
#[derive(Default)]
pub struct InMemorySessionRepository {
    sessions: HashMap<String, Session>,
}

impl SessionRepositoryPort for InMemorySessionRepository {
    fn save(&mut self, session: Session) {
        self.sessions.insert(session.session_id.clone(), session);
    }

    fn find_by_id(&self, session_id: &str) -> Option<Session> {
        self.sessions.get(session_id).cloned()
    }

    fn list(&self) -> Vec<Session> {
        let mut values = self.sessions.values().cloned().collect::<Vec<_>>();
        values.sort_by(|left, right| left.session_id.cmp(&right.session_id));
        values
    }
}

/// 会话事件时间线的内存查询模型。
#[derive(Default)]
pub struct InMemorySessionEventStore {
    events_by_session: HashMap<String, Vec<RuntimeEvent>>,
}

impl SessionEventStorePort for InMemorySessionEventStore {
    fn append(&mut self, session_id: &str, events: &[RuntimeEvent]) {
        self.events_by_session
            .entry(session_id.to_string())
            .or_default()
            .extend(events.iter().cloned());
    }

    fn list_by_session(&self, session_id: &str) -> Vec<RuntimeEvent> {
        self.events_by_session
            .get(session_id)
            .cloned()
            .unwrap_or_default()
    }
}
