import type { SessionApi } from '../api/sessions.ts';
import type { RuntimeEvent, Session } from '../../src/shared/session-runtime.ts';

export interface SessionStoreState {
  sessions: Session[];
  selectedSessionId?: string;
  selectedSession?: Session;
  events: RuntimeEvent[];
  pending: boolean;
  error?: string;
}

export class SessionStore {
  private readonly api: SessionApi;

  private state: SessionStoreState = {
    sessions: [],
    events: [],
    pending: false
  };

  private readonly listeners = new Set<(state: SessionStoreState) => void>();
  private eventUnsubscribe?: () => void;

  constructor(api: SessionApi) {
    this.api = api;
  }

  subscribe(listener: (state: SessionStoreState) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  getState(): SessionStoreState {
    return this.state;
  }

  async init() {
    await this.reloadSessions();
    if (!this.state.selectedSessionId && this.state.sessions.length > 0) {
      await this.selectSession(this.state.sessions[0].session_id);
    }
  }

  async reloadSessions() {
    this.setState({ pending: true, error: undefined });
    try {
      const sessions = await this.api.listSessions();
      this.setState({ sessions, pending: false });
      if (this.state.selectedSessionId) {
        const selected = sessions.find((session) => session.session_id === this.state.selectedSessionId);
        this.setState({ selectedSession: selected });
      }
    } catch (error) {
      this.setState({
        pending: false,
        error: error instanceof Error ? error.message : '加载会话失败'
      });
    }
  }

  async createSession(title: string) {
    this.setState({ pending: true, error: undefined });
    try {
      const session = await this.api.createSession(title);
      await this.reloadSessions();
      await this.selectSession(session.session_id);
    } catch (error) {
      this.setState({
        pending: false,
        error: error instanceof Error ? error.message : '创建会话失败'
      });
    }
  }

  async selectSession(sessionId: string) {
    this.eventUnsubscribe?.();
    this.setState({ pending: true, error: undefined });
    try {
      const [session, events] = await Promise.all([
        this.api.getSession(sessionId),
        this.api.listEventHistory(sessionId)
      ]);
      this.setState({
        selectedSessionId: sessionId,
        selectedSession: session,
        events,
        pending: false
      });
      this.eventUnsubscribe = this.api.subscribeSessionEvents(sessionId, (event) => {
        const selected = this.state.selectedSession;
        this.setState({
          events: [...this.state.events, event],
          selectedSession: selected
            ? {
                ...selected,
                status: resolveStatusFromEvent(selected.status, event.type)
              }
            : selected
        });
      });
    } catch (error) {
      this.setState({
        pending: false,
        error: error instanceof Error ? error.message : '加载会话详情失败'
      });
    }
  }

  async sendMessage(content: string, toolName?: string) {
    const sessionId = this.state.selectedSessionId;
    if (!sessionId) {
      this.setState({ error: '请先选择会话' });
      return;
    }

    this.setState({ pending: true, error: undefined });
    try {
      const result = await this.api.executeMessage(sessionId, {
        content,
        tool_name: toolName,
        tool_input: toolName ? { content } : undefined
      });
      await this.reloadSessions();
      this.setState({
        selectedSession: result.session,
        events: await this.api.listEventHistory(sessionId),
        pending: false
      });
    } catch (error) {
      this.setState({
        pending: false,
        error: error instanceof Error ? error.message : '执行失败'
      });
    }
  }

  private setState(patch: Partial<SessionStoreState>) {
    this.state = { ...this.state, ...patch };
    this.listeners.forEach((listener) => listener(this.state));
  }
}

function resolveStatusFromEvent(current: Session['status'], type: RuntimeEvent['type']): Session['status'] {
  if (type === 'execution_started') {
    return 'running';
  }
  if (type === 'execution_completed') {
    return 'completed';
  }
  if (type === 'execution_failed') {
    return 'failed';
  }
  return current;
}
