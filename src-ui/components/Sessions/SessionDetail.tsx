import { useState } from 'react';
import type { Session } from '../../../src/shared/session-runtime';

interface SessionDetailProps {
  session?: Session;
  pending: boolean;
  onSendMessage: (content: string, toolName?: string) => Promise<void>;
}

export function SessionDetail({ session, pending, onSendMessage }: SessionDetailProps) {
  const [message, setMessage] = useState('');
  const [toolName, setToolName] = useState('');

  if (!session) {
    return (
      <section className="panel detail-panel">
        <h3>会话详情</h3>
        <div className="empty">请选择一个会话</div>
      </section>
    );
  }

  return (
    <section className="panel detail-panel">
      <header className="detail-head">
        <div>
          <h3>{session.title}</h3>
          <small>{session.session_id}</small>
        </div>
        <span className={`status ${session.status}`}>{statusLabel(session.status)}</span>
      </header>

      <div className="detail-metrics">
        <div>
          <span>最近执行</span>
          <strong>{session.latest_execution?.status ?? 'idle'}</strong>
        </div>
        <div>
          <span>结果摘要</span>
          <strong>{session.latest_execution?.result_summary ?? '暂无'}</strong>
        </div>
      </div>

      <form
        className="message-form"
        onSubmit={async (event) => {
          event.preventDefault();
          if (!message.trim()) {
            return;
          }
          await onSendMessage(message, toolName.trim() || undefined);
          setMessage('');
        }}
      >
        <label>
          消息
          <textarea
            value={message}
            onChange={(event) => setMessage(event.target.value)}
            placeholder="输入消息并触发执行"
            rows={4}
          />
        </label>
        <label>
          工具名（可选，输入 fail 可模拟失败）
          <input
            value={toolName}
            onChange={(event) => setToolName(event.target.value)}
            placeholder="例如: uppercase"
          />
        </label>
        <button type="submit" className="primary" disabled={pending}>
          {pending ? '执行中...' : '发送并执行'}
        </button>
      </form>
    </section>
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
