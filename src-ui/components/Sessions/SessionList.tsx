import type { Session } from '../../../src/shared/session-runtime';

interface SessionListProps {
  sessions: Session[];
  selectedSessionId?: string;
  pending: boolean;
  onSelect: (sessionId: string) => void;
  onCreate: () => void;
}

export function SessionList({
  sessions,
  selectedSessionId,
  pending,
  onSelect,
  onCreate
}: SessionListProps) {
  return (
    <aside className="session-sidebar">
      <div className="session-sidebar-header">
        <h2>会话</h2>
        <button type="button" className="primary" onClick={onCreate} disabled={pending}>
          新建
        </button>
      </div>
      <div className="session-list">
        {sessions.length === 0 ? (
          <div className="empty">暂无会话</div>
        ) : (
          sessions.map((session) => (
            <button
              type="button"
              key={session.session_id}
              className={`session-item${session.session_id === selectedSessionId ? ' active' : ''}`}
              onClick={() => onSelect(session.session_id)}
            >
              <div>
                <strong>{session.title}</strong>
                <small>{session.session_id}</small>
              </div>
              <span className={`status ${session.status}`}>{statusLabel(session.status)}</span>
            </button>
          ))
        )}
      </div>
    </aside>
  );
}

function statusLabel(status: Session['status']) {
  if (status === 'running') {
    return '运行中';
  }
  if (status === 'completed') {
    return '已完成';
  }
  if (status === 'failed') {
    return '失败';
  }
  return '空闲';
}
