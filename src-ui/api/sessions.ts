import type {
  RuntimeEvent,
  RuntimeExecution,
  RuntimeExecutionStatus,
  Session,
  SessionMessageInput,
  ToolCall
} from '../../src/shared/session-runtime.ts';

type SessionEventListener = (event: RuntimeEvent) => void;

export interface SessionApi {
  createSession(title: string): Promise<Session>;
  listSessions(): Promise<Session[]>;
  getSession(sessionId: string): Promise<Session>;
  executeMessage(
    sessionId: string,
    message: SessionMessageInput
  ): Promise<{ session: Session; events: RuntimeEvent[]; toolCalls: ToolCall[] }>;
  listEventHistory(sessionId: string): Promise<RuntimeEvent[]>;
  subscribeSessionEvents(sessionId: string, listener: SessionEventListener): () => void;
}

type SessionState = Session & {
  events: RuntimeEvent[];
};

export function createLocalSessionApi(): SessionApi {
  const sessions = new Map<string, SessionState>();
  const listeners = new Map<string, Set<SessionEventListener>>();
  let sessionSeq = 1;
  let executionSeq = 1;
  let eventSeq = 1;
  let toolSeq = 1;

  const now = () => new Date().toISOString();

  const emitEvent = (sessionId: string, event: RuntimeEvent) => {
    const state = sessions.get(sessionId);
    if (!state) {
      return;
    }
    state.events.push(event);
    listeners.get(sessionId)?.forEach((listener) => listener(event));
  };

  const ensureSession = (sessionId: string): SessionState => {
    const state = sessions.get(sessionId);
    if (!state) {
      throw new Error(`SESSION_NOT_FOUND:${sessionId}`);
    }
    return state;
  };

  const createSeedSession = (title: string): SessionState => {
    const timestamp = now();
    return {
      session_id: `sess-${sessionSeq++}`,
      title,
      status: 'idle',
      created_at: timestamp,
      updated_at: timestamp,
      events: []
    };
  };

  const seed = createSeedSession('运行时验证会话');
  sessions.set(seed.session_id, seed);

  return {
    async createSession(title: string) {
      const state = createSeedSession(title || '新会话');
      sessions.set(state.session_id, state);
      return toSession(state);
    },

    async listSessions() {
      return Array.from(sessions.values())
        .sort((a, b) => b.updated_at.localeCompare(a.updated_at))
        .map(toSession);
    },

    async getSession(sessionId: string) {
      return toSession(ensureSession(sessionId));
    },

    async executeMessage(sessionId: string, message: SessionMessageInput) {
      const state = ensureSession(sessionId);
      if (state.status === 'running') {
        throw new Error(`SESSION_RUNNING:${sessionId}`);
      }

      const startedAt = now();
      const executionId = `exec-${executionSeq++}`;
      state.status = 'running';
      state.updated_at = startedAt;

      const startEvent = createEvent(sessionId, executionId, 'execution_started', {
        message: message.content
      });
      emitEvent(sessionId, startEvent);

      const events: RuntimeEvent[] = [startEvent];
      const toolCalls: ToolCall[] = [];
      let status: RuntimeExecutionStatus = 'completed';
      let resultSummary = `Echo: ${message.content}`;
      let errorMessage: string | undefined;

      await wait(180);

      if (message.tool_name) {
        const toolCallId = `tool-${toolSeq++}`;
        const toolStart = createEvent(sessionId, executionId, 'tool_call_started', {
          tool_call_id: toolCallId,
          tool_name: message.tool_name,
          input: message.tool_input || {}
        });
        emitEvent(sessionId, toolStart);
        events.push(toolStart);

        if (message.tool_name === 'fail') {
          status = 'failed';
          errorMessage = 'tool fail requested';
          const toolCall: ToolCall = {
            tool_call_id: toolCallId,
            execution_id: executionId,
            tool_name: message.tool_name,
            status: 'failed',
            input: message.tool_input || {},
            error_message: errorMessage,
            started_at: startedAt,
            completed_at: now()
          };
          toolCalls.push(toolCall);
        } else {
          const output = {
            ok: true,
            echo: message.tool_input || {}
          };
          resultSummary = `Tool ${message.tool_name} finished`;
          const toolComplete = createEvent(sessionId, executionId, 'tool_call_completed', {
            tool_call_id: toolCallId,
            tool_name: message.tool_name,
            output
          });
          emitEvent(sessionId, toolComplete);
          events.push(toolComplete);

          const toolCall: ToolCall = {
            tool_call_id: toolCallId,
            execution_id: executionId,
            tool_name: message.tool_name,
            status: 'completed',
            input: message.tool_input || {},
            output,
            started_at: startedAt,
            completed_at: now()
          };
          toolCalls.push(toolCall);
        }
      }

      const completedAt = now();
      const metrics = {
        latency_ms: 180,
        input_tokens: message.content.length,
        output_tokens: resultSummary.length
      };
      const latestExecution: RuntimeExecution = {
        execution_id: executionId,
        session_id: sessionId,
        input_message_id: `msg-${executionId}`,
        status,
        started_at: startedAt,
        completed_at: completedAt,
        result_summary: status === 'completed' ? resultSummary : undefined,
        error_message: errorMessage,
        metrics
      };

      const terminalEvent = createEvent(
        sessionId,
        executionId,
        status === 'completed' ? 'execution_completed' : 'execution_failed',
        status === 'completed'
          ? { result_summary: resultSummary, metrics }
          : { error_message: errorMessage }
      );
      emitEvent(sessionId, terminalEvent);
      events.push(terminalEvent);

      state.status = status;
      state.updated_at = completedAt;
      state.latest_execution = latestExecution;

      return {
        session: toSession(state),
        events,
        toolCalls
      };
    },

    async listEventHistory(sessionId: string) {
      return [...ensureSession(sessionId).events];
    },

    subscribeSessionEvents(sessionId: string, listener: SessionEventListener) {
      const existing = listeners.get(sessionId) || new Set<SessionEventListener>();
      existing.add(listener);
      listeners.set(sessionId, existing);
      return () => {
        existing.delete(listener);
        if (existing.size === 0) {
          listeners.delete(sessionId);
        }
      };
    }
  };

  function createEvent(
    sessionId: string,
    executionId: string,
    type: RuntimeEvent['type'],
    payload: Record<string, unknown>
  ): RuntimeEvent {
    return {
      event_id: `evt-${eventSeq++}`,
      execution_id: executionId,
      session_id: sessionId,
      type,
      payload,
      created_at: now()
    };
  }
}

function toSession(state: SessionState): Session {
  return {
    session_id: state.session_id,
    title: state.title,
    status: state.status,
    created_at: state.created_at,
    updated_at: state.updated_at,
    latest_execution: state.latest_execution
  };
}

function wait(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}
