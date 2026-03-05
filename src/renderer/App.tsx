const models = [
  { id: 'gpt-5', name: 'gpt-5.2-codex', mode: '高强度' },
  { id: 'sonnet', name: 'claude-sonnet-4.6', mode: '平衡' },
  { id: 'mini', name: 'mini', mode: '低成本' }
];

const sessions = [
  {
    id: 's-01',
    name: '政务助手联调',
    status: '运行中',
    tokens: '18.6k / 128k',
    model: 'gpt-5.2-codex'
  },
  {
    id: 's-02',
    name: 'MCP 工具接入',
    status: '空闲',
    tokens: '2.4k / 64k',
    model: 'claude-sonnet-4.6'
  },
  {
    id: 's-03',
    name: 'Agent Teams 计划',
    status: '等待中',
    tokens: '0 / 64k',
    model: 'mini'
  }
];

const teamAgents = [
  { name: '调度', status: '活跃', role: 'Leader' },
  { name: '前端', status: '执行中', role: 'Worker' },
  { name: '后端', status: '空闲', role: 'Worker' }
];

export default function App() {
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
            {sessions.map((session) => (
              <div key={session.id} className="session-card">
                <div>
                  <h3>{session.name}</h3>
                  <p>{session.model}</p>
                </div>
                <div className="session-meta">
                  <span className="badge">{session.status}</span>
                  <span className="muted">{session.tokens}</span>
                </div>
              </div>
            ))}
          </div>
        </aside>

        <section className="workspace">
          <div className="workspace-header">
            <div>
              <h1>会话状态</h1>
              <p>集中查看上下文、模型、运行指标</p>
            </div>
            <div className="quick-actions">
              <button className="ghost">同步</button>
              <button className="ghost">导出</button>
            </div>
          </div>

          <div className="status-grid">
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
