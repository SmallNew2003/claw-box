import { useEffect, useMemo, useState } from 'react';
import { createLocalSessionApi } from './api/sessions';
import { EventTimeline } from './components/Sessions/EventTimeline';
import { SessionDetail } from './components/Sessions/SessionDetail';
import { SessionList } from './components/Sessions/SessionList';
import { SessionStore, type SessionStoreState } from './stores/session-store';

export default function App() {
  const api = useMemo(() => createLocalSessionApi(), []);
  const store = useMemo(() => new SessionStore(api), [api]);
  const [state, setState] = useState<SessionStoreState>(store.getState());

  useEffect(() => {
    const unsubscribe = store.subscribe(setState);
    void store.init();
    return unsubscribe;
  }, [store]);

  return (
    <div className="app-shell">
      <header className="topbar">
        <div>
          <h1>Agent Runtime Foundation</h1>
          <p>最小闭环：创建会话、发送消息、执行完成、查看事件日志</p>
        </div>
      </header>

      {state.error ? <div className="error-banner">{state.error}</div> : null}

      <main className="layout-grid">
        <SessionList
          sessions={state.sessions}
          selectedSessionId={state.selectedSessionId}
          pending={state.pending}
          onSelect={(sessionId) => {
            void store.selectSession(sessionId);
          }}
          onCreate={() => {
            void store.createSession(`新会话 ${state.sessions.length + 1}`);
          }}
        />

        <div className="content-grid">
          <SessionDetail
            session={state.selectedSession}
            pending={state.pending}
            onSendMessage={async (content, toolName) => {
              await store.sendMessage(content, toolName);
            }}
          />
          <EventTimeline events={state.events} />
        </div>
      </main>
    </div>
  );
}
