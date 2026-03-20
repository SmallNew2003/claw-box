use crate::contexts::runtime::domain::RuntimeEvent;
use crate::contexts::session::domain::Session;

/// Session 聚合持久化端口。
pub trait SessionRepositoryPort {
    fn save(&mut self, session: Session);
    fn find_by_id(&self, session_id: &str) -> Option<Session>;
    fn list(&self) -> Vec<Session>;
}

/// 会话事件查询端口，支撑时间线读模型。
pub trait SessionEventStorePort {
    fn append(&mut self, session_id: &str, events: &[RuntimeEvent]);
    fn list_by_session(&self, session_id: &str) -> Vec<RuntimeEvent>;
}
