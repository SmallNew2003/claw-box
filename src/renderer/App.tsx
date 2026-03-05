import { useMemo, useState } from 'react';

type StatusId = 'running' | 'idle' | 'waiting' | 'paused';

const models = [
  { id: 'gpt-5', name: 'gpt-5.2-codex', mode: '高强度' },
  { id: 'sonnet', name: 'claude-sonnet-4.6', mode: '平衡' },
  { id: 'mini', name: 'mini', mode: '低成本' },
  { id: 'vision', name: 'gpt-4o', mode: '多模态' }
];

const sessions = [
  {
    id: 's-01',
    name: '政务助手联调',
    status: 'running',
    statusLabel: '运行中',
    tokens: '18.6k / 128k',
    modelId: 'gpt-5',
    model: 'gpt-5.2-codex',
    owner: '城市局',
    lastActive: '2 分钟前',
    latency: '1.2s'
  },
  {
    id: 's-02',
    name: 'MCP 工具接入',
    status: 'idle',
    statusLabel: '空闲',
    tokens: '2.4k / 64k',
    modelId: 'sonnet',
    model: 'claude-sonnet-4.6',
    owner: '工具组',
    lastActive: '20 分钟前',
    latency: '0.9s'
  },
  {
    id: 's-03',
    name: 'Agent Teams 计划',
    status: 'waiting',
    statusLabel: '等待中',
    tokens: '0 / 64k',
    modelId: 'mini',
    model: 'mini',
    owner: '规划组',
    lastActive: '1 小时前',
    latency: '—'
  },
  {
    id: 's-04',
    name: '政务知识库优化',
    status: 'paused',
    statusLabel: '已暂停',
    tokens: '46k / 128k',
    modelId: 'gpt-5',
    model: 'gpt-5.2-codex',
    owner: '政务办',
    lastActive: '昨天 22:14',
    latency: '1.6s'
  },
  {
    id: 's-05',
    name: '数据治理报表',
    status: 'running',
    statusLabel: '运行中',
    tokens: '9.2k / 64k',
    modelId: 'sonnet',
    model: 'claude-sonnet-4.6',
    owner: '数据台',
    lastActive: '5 分钟前',
    latency: '1.1s'
  },
  {
    id: 's-06',
    name: '知识问答回放',
    status: 'waiting',
    statusLabel: '等待中',
    tokens: '11.8k / 128k',
    modelId: 'gpt-5',
    model: 'gpt-5.2-codex',
    owner: '知识库',
    lastActive: '12 分钟前',
    latency: '1.4s'
  }
];

const teamAgents = [
  { name: '调度', status: '活跃', role: 'Leader' },
  { name: '前端', status: '执行中', role: 'Worker' },
  { name: '后端', status: '空闲', role: 'Worker' }
];

const statusCards: Array<{
  id: StatusId;
  label: string;
  hint: string;
  trend: string;
}> = [
  {
    id: 'running',
    label: '运行中',
    hint: '高优先级任务保持在 4 条以内',
    trend: '+2 今日'
  },
  {
    id: 'idle',
    label: '空闲',
    hint: '待调度会话可快速分配算力',
    trend: '-1 今日'
  },
  {
    id: 'waiting',
    label: '等待中',
    hint: '依赖外部输入，需补齐资料',
    trend: '+3 今日'
  },
  {
    id: 'paused',
    label: '已暂停',
    hint: '暂停会话建议 24h 内确认',
    trend: '1 待恢复'
  }
];

type SessionStatusCardProps = {
  label: string;
  count: number;
  hint: string;
  trend: string;
  tone: StatusId;
  active: boolean;
  onSelect: () => void;
};

function SessionStatusCard({
  label,
  count,
  hint,
  trend,
  tone,
  active,
  onSelect
}: SessionStatusCardProps) {
  return (
    <button
      type="button"
      className={`session-status-card${active ? ' is-active' : ''}`}
      aria-pressed={active}
      onClick={onSelect}
    >
      <div className="session-status-card-header">
        <span>{label}</span>
        <span className={`status-tone ${tone}`}>{trend}</span>
      </div>
      <strong className="session-status-count">{count}</strong>
      <p className="session-status-hint">{hint}</p>
    </button>
  );
}

export default function App() {
  const [statusFilter, setStatusFilter] = useState<'all' | StatusId>('all');
  const [modelFilter, setModelFilter] = useState<'all' | (typeof models)[number]['id']>('all');

  const filteredSessions = useMemo(
    () =>
      sessions.filter((session) => {
        if (statusFilter !== 'all' && session.status !== statusFilter) {
          return false;
        }
        if (modelFilter !== 'all' && session.modelId !== modelFilter) {
          return false;
        }
        return true;
      }),
    [statusFilter, modelFilter]
  );

  const statusCounts = useMemo(() => {
    const base = statusCards.reduce(
      (acc, card) => {
        acc[card.id] = 0;
        return acc;
      },
      {} as Record<StatusId, number>
    );
    sessions.forEach((session) => {
      if (modelFilter !== 'all' && session.modelId !== modelFilter) {
        return;
      }
      base[session.status] += 1;
    });
    return base;
  }, [modelFilter]);

  const statusFilterOptions = [
    { id: 'all', label: '全部状态' },
    ...statusCards.map((card) => ({ id: card.id, label: card.label }))
  ];

  const modelFilterOptions = [
    { id: 'all', name: '全部模型', mode: '全部' },
    ...models
  ];

  const handleStatusCardSelect = (nextStatus: StatusId) => {
    setStatusFilter((current) => (current === nextStatus ? 'all' : nextStatus));
  };

  return (
    <div className="app-shell">
      <header className="topbar">
        <div className="brand">
          <span className="brand-mark">Claw Box</span>
          <span className="brand-sub">OpenClaw 控制台</span>
        </div>
        <div className="model-switch">
          <label htmlFor="model" className="label">模型</label>
          <select id="model" className="select">
            {models.map((model) => (
              <option key={model.id} value={model.id}>
                {model.name} · {model.mode}
              </option>
            ))}
          </select>
          <button className="ghost">思考强度</button>
        </div>
        <div className="top-actions">
          <button className="pill">全局状态</button>
          <button className="pill">设置</button>
        </div>
      </header>

      <main className="main-grid">
        <aside className="sidebar">
          <div className="sidebar-header">
            <h2>会话</h2>
            <button className="primary">新建</button>
          </div>
          <input className="search" placeholder="搜索会话" />
          <div className="session-list">
            {filteredSessions.length === 0 ? (
              <div className="empty-state small">暂无匹配会话</div>
            ) : (
              filteredSessions.map((session) => (
                <div key={session.id} className="session-card">
                  <div>
                    <h3>{session.name}</h3>
                    <p>{session.model}</p>
                  </div>
                  <div className="session-meta">
                    <span className={`status-badge ${session.status}`}>{session.statusLabel}</span>
                    <span className="muted">{session.tokens}</span>
                  </div>
                </div>
              ))
            )}
          </div>
        </aside>

        <section className="workspace">
          <div className="dashboard-header">
            <div>
              <h1>会话仪表盘</h1>
              <p>集中查看会话运行状态、模型占用与执行指标</p>
            </div>
            <div className="quick-actions">
              <button className="ghost">同步</button>
              <button className="ghost">导出</button>
            </div>
          </div>

          <div className="dashboard-controls">
            <div className="filter-group">
              <label className="label" htmlFor="statusFilter">状态</label>
              <select
                id="statusFilter"
                className="select filter-select"
                value={statusFilter}
                onChange={(event) =>
                  setStatusFilter(event.target.value as 'all' | StatusId)
                }
              >
                {statusFilterOptions.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.label}
                  </option>
                ))}
              </select>
            </div>
            <div className="filter-group">
              <label className="label" htmlFor="modelFilter">模型</label>
              <select
                id="modelFilter"
                className="select filter-select"
                value={modelFilter}
                onChange={(event) =>
                  setModelFilter(event.target.value as 'all' | (typeof models)[number]['id'])
                }
              >
                {modelFilterOptions.map((option) => (
                  <option key={option.id} value={option.id}>
                    {option.name} · {option.mode}
                  </option>
                ))}
              </select>
            </div>
            <div className="filter-summary">
              <span className="label">已匹配</span>
              <strong>{filteredSessions.length}</strong>
              <span className="muted">/ {sessions.length} 会话</span>
            </div>
          </div>

          <div className="session-status-grid">
            {statusCards.map((card) => (
              <SessionStatusCard
                key={card.id}
                label={card.label}
                count={statusCounts[card.id]}
                hint={card.hint}
                trend={card.trend}
                tone={card.id}
                active={statusFilter === card.id}
                onSelect={() => handleStatusCardSelect(card.id)}
              />
            ))}
          </div>

          <div className="session-table">
            <div className="session-table-header">
              <span>会话</span>
              <span>状态</span>
              <span>模型</span>
              <span>Token 使用</span>
              <span>最近活动</span>
            </div>
            {filteredSessions.length === 0 ? (
              <div className="empty-state">暂无符合筛选条件的会话</div>
            ) : (
              filteredSessions.map((session) => (
                <div key={session.id} className="session-row">
                  <div className="session-title">
                    <strong>{session.name}</strong>
                    <span className="muted">{session.owner}</span>
                  </div>
                  <span className={`status-badge ${session.status}`}>{session.statusLabel}</span>
                  <span>{session.model}</span>
                  <span>{session.tokens}</span>
                  <span className="muted">{session.lastActive}</span>
                </div>
              ))
            )}
          </div>

          <div className="overview-grid">
            <div className="status-card">
              <h4>上下文概览</h4>
              <div className="metric">
                <span>Token 使用</span>
                <strong>54k / 256k</strong>
              </div>
              <div className="metric">
                <span>消息数</span>
                <strong>128</strong>
              </div>
              <div className="metric">
                <span>平均响应</span>
                <strong>1.2s</strong>
              </div>
            </div>
            <div className="status-card">
              <h4>Agent Teams</h4>
              <div className="team-list">
                {teamAgents.map((agent) => (
                  <div key={agent.name} className="team-row">
                    <span>{agent.name}</span>
                    <span className="muted">{agent.role}</span>
                    <span className="badge subtle">{agent.status}</span>
                  </div>
                ))}
              </div>
            </div>
            <div className="status-card">
              <h4>定时任务</h4>
              <div className="metric">
                <span>运行中</span>
                <strong>2</strong>
              </div>
              <div className="metric">
                <span>计划任务</span>
                <strong>8</strong>
              </div>
              <button className="link">管理任务</button>
            </div>
          </div>
        </section>
      </main>

      <footer className="footer">
        <div className="footer-group">
          <button className="footer-item">任务</button>
          <button className="footer-item">技能</button>
          <button className="footer-item">MCP</button>
        </div>
        <div className="footer-group">
          <button className="footer-item">Teams</button>
          <button className="footer-item">项目</button>
          <button className="footer-item">市场</button>
        </div>
      </footer>
    </div>
  );
}
