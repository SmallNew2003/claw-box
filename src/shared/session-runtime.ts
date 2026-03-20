// 运行时执行状态，前后端共用。
export type RuntimeExecutionStatus = 'idle' | 'running' | 'completed' | 'failed';

// 运行时事件类型，保证事件时间线的一致枚举。
export type RuntimeEventType =
  | 'execution_started'
  | 'step_progress'
  | 'tool_call_started'
  | 'tool_call_completed'
  | 'execution_completed'
  | 'execution_failed';

export interface RuntimeMetrics {
  latency_ms: number;
  input_tokens: number;
  output_tokens: number;
}

export interface RuntimeExecution {
  execution_id: string;
  session_id: string;
  input_message_id: string;
  status: RuntimeExecutionStatus;
  started_at: string;
  completed_at?: string;
  result_summary?: string;
  error_message?: string;
  metrics?: RuntimeMetrics;
}

export interface RuntimeEvent {
  event_id: string;
  execution_id: string;
  session_id: string;
  type: RuntimeEventType;
  payload: Record<string, unknown>;
  created_at: string;
}

export interface ToolCall {
  tool_call_id: string;
  execution_id: string;
  tool_name: string;
  status: 'running' | 'completed' | 'failed';
  input: Record<string, unknown>;
  output?: Record<string, unknown>;
  error_message?: string;
  started_at: string;
  completed_at?: string;
}

// 会话聚合视图，包含最近一次执行摘要。
export interface Session {
  session_id: string;
  title: string;
  status: RuntimeExecutionStatus;
  created_at: string;
  updated_at: string;
  latest_execution?: RuntimeExecution;
}

export interface SessionMessageInput {
  content: string;
  tool_name?: string;
  tool_input?: Record<string, unknown>;
}
