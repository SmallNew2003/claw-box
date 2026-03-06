# Claw Box 架构设计

## 文档索引

- [后端架构设计](./backend-architecture.md) - Core Service 详细设计
- [API 规范](./api-specification.md) - HTTP API 与事件协议
- [事件协议](./event-protocol.md) - SSE 事件流设计
- [安全设计](./security-design.md) - 安全策略与实现
- [性能策略](./performance-strategy.md) - 性能优化方案

## 整体架构

Claw Box 采用前后端分离架构，前端为桌面控制台（Tauri/Electron），后端为 Core Service（Rust），通过 HTTP API 与事件流通信。

```
┌─────────────────────────────────────────────────────┐
│                   Claw Box 系统                      │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌──────────────┐         ┌──────────────────┐    │
│  │  前端控制台   │  HTTP   │   Core Service   │    │
│  │ (Tauri/React)│◄───────►│     (Rust)       │    │
│  │              │  SSE    │                  │    │
│  │ - UI 组件     │         │ - 业务逻辑        │    │
│  │ - 状态管理    │         │ - 任务编排        │    │
│  │ - 用户交互    │         │ - 数据持久化      │    │
│  │ - 数据展示    │         │ - 模型调用        │    │
│  └──────────────┘         └──────────────────┘    │
│                                 │                  │
│                                 ▼                  │
│                          ┌──────────────┐         │
│                          │   SQLite     │         │
│                          │ + 向量检索    │         │
│                          └──────────────┘         │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## 技术栈

### 前端（控制台）
- **Tauri**（优先）或 **Electron**: 桌面应用框架
- **React**: UI 框架，版本 18+
- **TypeScript**: 类型系统，版本 5+
- **Vite**: 构建工具和开发服务器

### 后端（Core Service）
- **Rust**: 业务逻辑与 API 服务
- **Axum/Actix-web**: HTTP 框架
- **SQLite**: 默认数据库
- **本地向量检索**: 记忆系统（可选外部 Qdrant/pgvector）

### UI 和样式
- **TailwindCSS**: 原子化 CSS 框架
- **Lucide React**: 图标库

### 状态管理
- **Zustand**: 轻量级状态管理
- **React Query**: 服务端状态管理

### 数据持久化
- **SQLite**: 默认存储（会话、记忆、配置）
- **可选**: PostgreSQL + Qdrant/pgvector + Redis

## 目录结构

```
claw-box/
├── frontend/                   # 前端控制台
│   ├── src/
│   │   ├── App.tsx            # 应用根组件
│   │   ├── main.tsx           # 入口
│   │   ├── components/        # UI 组件
│   │   │   ├── Session/       # 会话组件
│   │   │   ├── AgentTeams/    # Agent Teams 组件
│   │   │   ├── Memory/        # 记忆组件
│   │   │   ├── Skills/        # 技能组件
│   │   │   └── MCP/           # MCP 组件
│   │   ├── stores/            # 状态管理
│   │   ├── api/               # API 客户端
│   │   ├── hooks/             # 自定义 Hooks
│   │   └── types/             # 类型定义
│   ├── index.html
│   └── vite.config.ts
│
├── backend/                    # Core Service (Rust)
│   ├── src/
│   │   ├── main.rs            # 入口
│   │   ├── api/               # HTTP API
│   │   ├── services/          # 业务逻辑
│   │   │   ├── session.rs     # 会话服务
│   │   │   ├── agent_teams.rs # Agent Teams 服务
│   │   │   ├── memory.rs      # 记忆服务
│   │   │   ├── skills.rs      # 技能服务
│   │   │   └── mcp.rs         # MCP 服务
│   │   ├── models/            # 数据模型
│   │   ├── db/                # 数据库层
│   │   └── orchestrator/      # 任务编排
│   └── Cargo.toml
│
├── docs/                       # 文档
├── openspec/                   # OpenSpec 规范
└── tools/                      # 工具脚本
```

## 核心模块设计

### 1. 会话管理模块

**职责**
- 会话的 CRUD 操作
- 会话状态维护
- 消息历史管理
- 运行态（Run/Step）展示

**数据流**
```
UI 组件 → API 客户端 → Core Service → SQLite
                  ↓
              SSE 推送
                  ↓
              更新 UI
```

**关键接口**
```typescript
interface Session {
  id: string;
  name: string;
  modelId: string;
  messages: Message[];
  createdAt: number;
  updatedAt: number;
  status: SessionStatus;
}

interface SessionAPI {
  create(name: string): Promise<Session>;
  delete(id: string): Promise<void>;
  update(id: string, data: Partial<Session>): Promise<Session>;
  list(): Promise<Session[]>;
  get(id: string): Promise<Session>;
}
```

### 2. Agent Teams 模块

**职责**
- 团队编排与角色管理
- 任务分配与协作流程
- 运行态可视化
- 结果聚合

**团队结构**
```typescript
interface AgentTeam {
  id: string;
  name: string;
  mode: 'sequential' | 'parallel' | 'expert' | 'pipeline';
  roles: Role[];
  flow: Flow;
  currentRun?: Run;
}

interface Run {
  id: string;
  teamId: string;
  status: 'running' | 'completed' | 'failed';
  steps: Step[];
  startedAt: number;
  completedAt?: number;
}
```

### 3. 记忆系统模块

**职责**
- 长期记忆存储与检索
- 向量 embedding 生成
- 语义相似度匹配
- 记忆管理（标签、编辑、删除）

**记忆结构**
```typescript
interface MemoryItem {
  id: string;
  content: string;
  summary: string;
  embedding: number[];
  tags: string[];
  source: string;
  createdAt: number;
}

interface MemoryAPI {
  store(content: string, tags?: string[]): Promise<MemoryItem>;
  search(query: string, limit?: number): Promise<MemoryItem[]>;
  list(filters?: MemoryFilters): Promise<MemoryItem[]>;
  update(id: string, data: Partial<MemoryItem>): Promise<MemoryItem>;
  delete(id: string): Promise<void>;
}
```

### 4. 技能管理模块

**职责**
- 技能安装与配置
- 技能执行监控
- 技能版本管理
- 权限控制

**技能接口**
```typescript
interface Skill {
  id: string;
  name: string;
  version: string;
  description: string;
  config?: SkillConfig;
  permissions: string[];
}

interface SkillAPI {
  install(packagePath: string): Promise<Skill>;
  uninstall(id: string): Promise<void>;
  list(): Promise<Skill[]>;
  configure(id: string, config: SkillConfig): Promise<void>;
  execute(id: string, params: any): Promise<any>;
}
```

### 5. MCP 集成模块

**职责**
- MCP 服务器连接管理
- 工具和资源索引
- 调用代理与监控
- 日志记录

**MCP 连接**
```typescript
interface MCPConnection {
  id: string;
  name: string;
  url: string;
  status: 'connected' | 'disconnected';
  tools: MCPTool[];
  resources: MCPResource[];
}

interface MCPAPI {
  connect(config: MCPConfig): Promise<MCPConnection>;
  disconnect(id: string): Promise<void>;
  listTools(connectionId: string): Promise<MCPTool[]>;
  callTool(connectionId: string, toolName: string, params: any): Promise<any>;
}
```

## API 通信设计

### 通信模式

1. **HTTP REST API**
   - 前端发送请求到 Core Service
   - 用于 CRUD 操作与查询
   - JSON 格式数据交换

2. **Server-Sent Events (SSE)**
   - Core Service 推送实时事件到前端
   - 用于运行态更新、日志流、状态变化
   - 单向推送，低延迟

### API 端点定义

```typescript
// 会话相关
GET    /api/sessions
POST   /api/sessions
GET    /api/sessions/:id
PUT    /api/sessions/:id
DELETE /api/sessions/:id

// Agent Teams 相关
GET    /api/teams
POST   /api/teams
GET    /api/teams/:id/runs
POST   /api/teams/:id/runs

// 记忆相关
GET    /api/memories
POST   /api/memories
GET    /api/memories/search?q=...
PUT    /api/memories/:id
DELETE /api/memories/:id

// 技能相关
GET    /api/skills
POST   /api/skills/install
DELETE /api/skills/:id
POST   /api/skills/:id/execute

// MCP 相关
GET    /api/mcp/connections
POST   /api/mcp/connections
GET    /api/mcp/connections/:id/tools
POST   /api/mcp/connections/:id/call

// 事件流
GET    /api/events (SSE)
```

## 数据存储设计

### 存储策略

1. **SQLite（默认）**
   - 会话数据
   - 记忆条目（元数据）
   - 技能配置
   - MCP 连接配置
   - 运行日志

2. **本地向量检索**
   - 记忆 embedding 存储与检索
   - 可选外部 Qdrant/pgvector

3. **可选外部存储**
   - PostgreSQL（替代 SQLite）
   - Redis（缓存与会话状态）

### 数据目录结构

```
~/.claw-box/                    # 用户数据目录
├── config.toml                 # 应用配置
├── claw-box.db                 # SQLite 数据库
├── vectors/                    # 本地向量索引
└── logs/                       # 日志文件
```

## 性能优化

### 渲染优化
- 虚拟滚动（会话列表）
- 消息懒加载
- React.memo 优化重渲染
- 防抖和节流

### 内存优化
- 会话数据分页加载
- 旧会话自动归档
- 定期清理缓存
- 限制消息历史长度

### 启动优化
- 延迟加载非关键模块
- 预加载常用数据
- 优化打包体积
- 使用 V8 快照

## 安全考虑

### 数据安全
- API Key 加密存储
- 敏感数据不记录日志
- 本地数据权限控制

### 进程隔离
- 禁用 Node.js 集成（渲染进程）
- 使用 contextBridge
- 严格的 CSP 策略

### 更新安全
- 代码签名
- 自动更新验证
- 回滚机制

## 扩展性设计

### 插件系统
- 插件 API 定义
- 插件生命周期管理
- 插件沙箱隔离

### 主题系统
- CSS 变量
- 主题配置文件
- 动态主题切换

### 国际化
- i18n 框架集成
- 语言包管理
- 动态语言切换
