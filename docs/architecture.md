# Claw Box 架构设计

## 整体架构

Claw Box 采用 Electron 多进程架构，分为主进程和渲染进程，通过 IPC 通信实现数据交互。

```
┌─────────────────────────────────────────────────────┐
│                   Electron 应用                      │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌──────────────┐         ┌──────────────────┐    │
│  │   主进程      │  IPC    │   渲染进程        │    │
│  │  (Node.js)   │◄───────►│   (React)        │    │
│  │              │         │                  │    │
│  │ - 窗口管理    │         │ - UI 组件         │    │
│  │ - 文件系统    │         │ - 状态管理        │    │
│  │ - 数据持久化  │         │ - 用户交互        │    │
│  │ - 系统集成    │         │ - 数据展示        │    │
│  └──────────────┘         └──────────────────┘    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## 技术栈

### 核心框架
- **Electron**: 桌面应用框架，版本 28+
- **React**: UI 框架，版本 18+
- **TypeScript**: 类型系统，版本 5+
- **Vite**: 构建工具和开发服务器

### UI 和样式
- **TailwindCSS**: 原子化 CSS 框架
- **Lucide React**: 图标库
- **Headless UI**: 无样式组件库（可选）

### 状态管理
- **Zustand**: 轻量级状态管理
- **React Context**: 局部状态共享

### 数据持久化
- **electron-store**: 配置存储
- **IndexedDB**: 大量数据存储
- **JSON 文件**: 会话数据存储

## 目录结构

```
claw-box/
├── src/
│   ├── main/                    # 主进程代码
│   │   ├── index.ts            # 主进程入口
│   │   ├── window.ts           # 窗口管理
│   │   ├── ipc/                # IPC 处理器
│   │   │   ├── session.ts      # 会话相关
│   │   │   ├── model.ts        # 模型相关
│   │   │   └── task.ts         # 任务相关
│   │   └── services/           # 后端服务
│   │       ├── storage.ts      # 数据存储
│   │       └── config.ts       # 配置管理
│   │
│   ├── renderer/               # 渲染进程代码
│   │   ├── src/
│   │   │   ├── App.tsx         # 应用根组件
│   │   │   ├── main.tsx        # 渲染进程入口
│   │   │   ├── components/     # UI 组件
│   │   │   │   ├── Layout/     # 布局组件
│   │   │   │   ├── Session/    # 会话组件
│   │   │   │   ├── Model/      # 模型组件
│   │   │   │   └── Status/     # 状态组件
│   │   │   ├── stores/         # 状态管理
│   │   │   │   ├── session.ts  # 会话状态
│   │   │   │   ├── model.ts    # 模型状态
│   │   │   │   └── ui.ts       # UI 状态
│   │   │   ├── hooks/          # 自定义 Hooks
│   │   │   ├── utils/          # 工具函数
│   │   │   └── types/          # 类型定义
│   │   ├── index.html          # HTML 模板
│   │   └── vite.config.ts      # Vite 配置
│   │
│   ├── shared/                 # 共享代码
│   │   ├── types/              # 共享类型
│   │   │   ├── session.ts      # 会话类型
│   │   │   ├── model.ts        # 模型类型
│   │   │   ├── task.ts         # 任务类型
│   │   │   └── ipc.ts          # IPC 类型
│   │   └── constants/          # 常量定义
│   │
│   └── preload/                # 预加载脚本
│       └── index.ts            # 暴露安全的 API
│
├── docs/                       # 文档
├── resources/                  # 资源文件
├── dist/                       # 构建输出
├── package.json
├── tsconfig.json
├── vite.config.ts
└── electron-builder.yml        # 打包配置
```

## 核心模块设计

### 1. 会话管理模块

**职责**
- 会话的 CRUD 操作
- 会话状态维护
- 消息历史管理
- 会话搜索和过滤

**数据流**
```
UI 组件 → Store → IPC → 主进程 → 文件系统
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

interface SessionService {
  create(name: string): Promise<Session>;
  delete(id: string): Promise<void>;
  update(id: string, data: Partial<Session>): Promise<Session>;
  list(): Promise<Session[]>;
  get(id: string): Promise<Session>;
}
```

### 2. 模型管理模块

**职责**
- 模型配置管理
- 模型状态监控
- 模型切换逻辑
- API 调用封装

**支持的模型**
```typescript
type ModelProvider = 'claude' | 'openai' | 'local' | 'custom';

interface Model {
  id: string;
  name: string;
  provider: ModelProvider;
  config: ModelConfig;
  status: 'available' | 'unavailable';
  metrics: ModelMetrics;
}
```

### 3. 状态监控模块

**职责**
- 实时状态收集
- 性能指标计算
- 状态变化通知
- 历史数据记录

**监控指标**
```typescript
interface SessionMetrics {
  tokenUsage: {
    input: number;
    output: number;
    total: number;
  };
  responseTime: number;
  messageCount: number;
  errorCount: number;
}
```

### 4. 任务调度模块

**职责**
- 定时任务创建和管理
- Cron 表达式解析
- 任务执行调度
- 任务结果处理

**任务结构**
```typescript
interface Task {
  id: string;
  name: string;
  schedule: string; // Cron 表达式
  action: TaskAction;
  enabled: boolean;
  lastRun?: number;
  nextRun?: number;
}
```

### 5. 技能系统模块

**职责**
- 技能注册和发现
- 技能执行引擎
- 技能依赖管理
- 技能配置存储

**技能接口**
```typescript
interface Skill {
  id: string;
  name: string;
  version: string;
  description: string;
  execute: (params: any) => Promise<any>;
  config?: SkillConfig;
}
```

### 6. MCP 集成模块

**职责**
- MCP 服务器连接
- 工具和资源管理
- 调用代理和监控
- 权限控制

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
```

### 7. Agent Teams 模块

**职责**
- 多 Agent 协调
- 任务分配策略
- 状态同步
- 结果聚合

**团队结构**
```typescript
interface AgentTeam {
  id: string;
  name: string;
  mode: 'leader-follower' | 'parallel' | 'expert' | 'pipeline';
  agents: Agent[];
  currentTask?: TeamTask;
}
```

### 8. 项目管理模块

**职责**
- 项目配置管理
- 文件关联
- 上下文维护
- 项目模板

**项目结构**
```typescript
interface Project {
  id: string;
  name: string;
  description: string;
  path: string;
  sessions: string[];
  config: ProjectConfig;
}
```

### 9. 技能市场模块

**职责**
- 技能浏览和搜索
- 技能安装和更新
- 本地技能发布
- 版本管理

**市场接口**
```typescript
interface SkillMarket {
  search(query: string): Promise<Skill[]>;
  install(skillId: string): Promise<void>;
  uninstall(skillId: string): Promise<void>;
  update(skillId: string): Promise<void>;
}
```

## IPC 通信设计

### 通信模式

1. **请求-响应模式**
   - 渲染进程发送请求
   - 主进程处理并返回结果
   - 用于 CRUD 操作

2. **事件订阅模式**
   - 主进程发送事件
   - 渲染进程监听并更新 UI
   - 用于状态变化通知

### IPC 通道定义

```typescript
// 会话相关
'session:create'
'session:delete'
'session:update'
'session:list'
'session:get'

// 模型相关
'model:list'
'model:switch'
'model:config'

// 任务相关
'task:create'
'task:list'
'task:execute'

// 事件通知
'session:updated'
'model:status-changed'
'task:completed'
```

## 数据存储设计

### 存储策略

1. **配置数据**: electron-store
   - 用户设置
   - 模型配置
   - 应用偏好

2. **会话数据**: JSON 文件
   - 每个会话一个文件
   - 便于导入导出
   - 支持版本控制

3. **大量数据**: IndexedDB
   - 消息历史
   - 任务记录
   - 性能指标

### 数据目录结构

```
~/Library/Application Support/claw-box/  (macOS)
├── config.json                          # 应用配置
├── sessions/                            # 会话数据
│   ├── session-1.json
│   ├── session-2.json
│   └── ...
├── tasks/                               # 任务数据
├── skills/                              # 技能数据
└── projects/                            # 项目数据
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
