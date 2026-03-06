# Performance Strategy - 性能指标与优化方案

<!-- 版本: v1.0.0 -->
<!-- 最后更新: 2026-03-06 -->
<!-- 维护者: 开发团队 -->

## 概述

本文档定义 Claw Box 的性能目标、测量方法、优化策略和监控方案。

### 性能目标

- 应用启动时间 < 2 秒
- UI 响应时间 < 100ms
- API 请求延迟 < 500ms (P95)
- 消息流式输出延迟 < 50ms
- 内存占用 < 500MB（空闲状态）
- 数据库查询 < 50ms (P95)
- 向量检索 < 100ms (P95)

## 性能指标

### 前端性能指标

#### 启动性能

| 指标 | 目标 | 测量方法 |
|------|------|---------|
| 首次内容绘制 (FCP) | < 1s | Performance API |
| 最大内容绘制 (LCP) | < 2s | Performance API |
| 首次输入延迟 (FID) | < 100ms | Performance API |
| 累积布局偏移 (CLS) | < 0.1 | Performance API |

测量代码：
```typescript
// 测量 FCP
const fcpObserver = new PerformanceObserver((list) => {
  for (const entry of list.getEntries()) {
    if (entry.name === 'first-contentful-paint') {
      console.log('FCP:', entry.startTime);
    }
  }
});
fcpObserver.observe({ entryTypes: ['paint'] });

// 测量 LCP
const lcpObserver = new PerformanceObserver((list) => {
  const entries = list.getEntries();
  const lastEntry = entries[entries.length - 1];
  console.log('LCP:', lastEntry.startTime);
});
lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });
```

#### 运行时性能

| 指标 | 目标 | 测量方法 |
|------|------|---------|
| 帧率 (FPS) | > 60 | requestAnimationFrame |
| 内存占用 | < 500MB | performance.memory |
| 组件渲染时间 | < 16ms | React Profiler |
| 状态更新延迟 | < 50ms | 自定义计时器 |

测量代码：
```typescript
// 测量 FPS
let lastTime = performance.now();
let frames = 0;

function measureFPS() {
  frames++;
  const currentTime = performance.now();
  
  if (currentTime >= lastTime + 1000) {
    const fps = Math.round((frames * 1000) / (currentTime - lastTime));
    console.log('FPS:', fps);
    frames = 0;
    lastTime = currentTime;
  }
  
  requestAnimationFrame(measureFPS);
}

measureFPS();

// 测量内存
if (performance.memory) {
  console.log('Memory:', {
    used: performance.memory.usedJSHeapSize / 1024 / 1024,
    total: performance.memory.totalJSHeapSize / 1024 / 1024,
    limit: performance.memory.jsHeapSizeLimit / 1024 / 1024,
  });
}
```

### 后端性能指标

#### API 性能

| 指标 | 目标 | 测量方法 |
|------|------|---------|
| 请求延迟 (P50) | < 100ms | 中间件计时 |
| 请求延迟 (P95) | < 500ms | 中间件计时 |
| 请求延迟 (P99) | < 1s | 中间件计时 |
| 吞吐量 | > 1000 req/s | 压力测试 |
| 错误率 | < 0.1% | 错误日志统计 |

Rust 实现：
```rust
use std::time::Instant;
use axum::{
    middleware::{self, Next},
    response::Response,
    http::Request,
};

async fn timing_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let start = Instant::now();
    let path = req.uri().path().to_string();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    tracing::info!(
        "Request {} completed in {:?}",
        path,
        duration
    );
    
    response
}
```

#### 数据库性能

| 指标 | 目标 | 测量方法 |
|------|------|---------|
| 查询延迟 (P95) | < 50ms | SQLx 日志 |
| 连接池利用率 | < 80% | 连接池监控 |
| 事务延迟 | < 100ms | 自定义计时 |
| 索引命中率 | > 95% | EXPLAIN QUERY PLAN |

测量代码：
```rust
use sqlx::sqlite::SqlitePool;
use std::time::Instant;

pub async fn measure_query_performance(pool: &SqlitePool) {
    let start = Instant::now();
    
    let result = sqlx::query("SELECT * FROM sessions WHERE status = ?")
        .bind("active")
        .fetch_all(pool)
        .await;
    
    let duration = start.elapsed();
    tracing::info!("Query completed in {:?}", duration);
    
    if duration.as_millis() > 50 {
        tracing::warn!("Slow query detected: {:?}", duration);
    }
}
```

#### 向量检索性能

| 指标 | 目标 | 测量方法 |
|------|------|---------|
| 检索延迟 (P95) | < 100ms | 自定义计时 |
| 索引构建时间 | < 10s (10K 条目) | 自定义计时 |
| 内存占用 | < 200MB (10K 条目) | 系统监控 |
| 召回率 | > 95% | 离线评估 |

## 优化策略

### 前端优化

#### 代码分割

使用 React.lazy 和动态导入：
```typescript
import { lazy, Suspense } from 'react';

const SessionList = lazy(() => import('./components/SessionList'));
const AgentTeams = lazy(() => import('./components/AgentTeams'));

function App() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <SessionList />
      <AgentTeams />
    </Suspense>
  );
}
```

#### 虚拟滚动

使用 react-window 处理长列表：
```typescript
import { FixedSizeList } from 'react-window';

function MessageList({ messages }: { messages: Message[] }) {
  return (
    <FixedSizeList
      height={600}
      itemCount={messages.length}
      itemSize={80}
      width="100%"
    >
      {({ index, style }) => (
        <div style={style}>
          {messages[index].content}
        </div>
      )}
    </FixedSizeList>
  );
}
```

#### 状态管理优化

使用 Zustand 的选择器避免不必要的重渲染：
```typescript
import create from 'zustand';

const useStore = create((set) => ({
  sessions: [],
  addSession: (session) => set((state) => ({
    sessions: [...state.sessions, session]
  })),
}));

// 只订阅需要的数据
function SessionCount() {
  const count = useStore((state) => state.sessions.length);
  return <div>Sessions: {count}</div>;
}
```

#### 防抖与节流

```typescript
import { debounce } from 'lodash-es';

const handleSearch = debounce((query: string) => {
  // 搜索逻辑
}, 300);

const handleScroll = throttle(() => {
  // 滚动逻辑
}, 100);
```

### 后端优化

#### 数据库优化

索引策略：
```sql
-- 会话查询索引
CREATE INDEX idx_sessions_status ON sessions(status);
CREATE INDEX idx_sessions_created_at ON sessions(created_at DESC);

-- 消息查询索引
CREATE INDEX idx_messages_session_id ON messages(session_id);
CREATE INDEX idx_messages_timestamp ON messages(timestamp DESC);

-- 复合索引
CREATE INDEX idx_sessions_status_created ON sessions(status, created_at DESC);
```

连接池配置：
```rust
use sqlx::sqlite::SqlitePoolOptions;

let pool = SqlitePoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .acquire_timeout(std::time::Duration::from_secs(5))
    .connect("sqlite:claw-box.db")
    .await?;
```

批量操作：
```rust
// 批量插入消息
pub async fn batch_insert_messages(
    pool: &SqlitePool,
    messages: Vec<Message>,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    
    for message in messages {
        sqlx::query("INSERT INTO messages (id, session_id, content) VALUES (?, ?, ?)")
            .bind(&message.id)
            .bind(&message.session_id)
            .bind(&message.content)
            .execute(&mut tx)
            .await?;
    }
    
    tx.commit().await?;
    Ok(())
}
```

#### 缓存策略

内存缓存：
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Cache<K, V> {
    data: Arc<RwLock<HashMap<K, V>>>,
    max_size: usize,
}

impl<K: Eq + std::hash::Hash, V: Clone> Cache<K, V> {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }
    
    pub async fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read().await;
        data.get(key).cloned()
    }
    
    pub async fn set(&self, key: K, value: V) {
        let mut data = self.data.write().await;
        
        if data.len() >= self.max_size {
            // LRU 淘汰策略（简化版）
            if let Some(first_key) = data.keys().next().cloned() {
                data.remove(&first_key);
            }
        }
        
        data.insert(key, value);
    }
}
```

#### 异步处理

使用 Tokio 并发处理：
```rust
use tokio::task;

pub async fn process_multiple_sessions(
    session_ids: Vec<String>,
) -> Vec<Result<Session, String>> {
    let tasks: Vec<_> = session_ids
        .into_iter()
        .map(|id| {
            task::spawn(async move {
                fetch_session(&id).await
            })
        })
        .collect();
    
    let results = futures::future::join_all(tasks).await;
    
    results
        .into_iter()
        .map(|r| r.unwrap())
        .collect()
}
```

#### 流式处理

SSE 流式输出优化：
```rust
use axum::response::sse::{Event, Sse};
use futures::stream::Stream;

pub async fn stream_messages(
    session_id: String,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        loop {
            let message = fetch_next_message(&session_id).await;
            
            yield Ok(Event::default()
                .event("message")
                .data(serde_json::to_string(&message).unwrap()));
            
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    };
    
    Sse::new(stream)
}
```

### 向量检索优化

#### HNSW 索引

使用 HNSW 算法构建索引：
```rust
use hnsw::{Hnsw, Params};

pub struct VectorIndex {
    index: Hnsw<f32, DistanceCosine>,
}

impl VectorIndex {
    pub fn new(dimension: usize) -> Self {
        let params = Params::new()
            .ef_construction(200)
            .m(16);
        
        let index = Hnsw::new(params);
        
        Self { index }
    }
    
    pub fn add(&mut self, id: usize, vector: &[f32]) {
        self.index.insert(id, vector);
    }
    
    pub fn search(&self, query: &[f32], k: usize) -> Vec<(usize, f32)> {
        self.index.search(query, k)
    }
}
```

#### 批量检索

```rust
pub async fn batch_search(
    queries: Vec<Vec<f32>>,
    k: usize,
) -> Vec<Vec<(usize, f32)>> {
    let tasks: Vec<_> = queries
        .into_iter()
        .map(|query| {
            task::spawn(async move {
                search_vector(&query, k).await
            })
        })
        .collect();
    
    let results = futures::future::join_all(tasks).await;
    
    results
        .into_iter()
        .map(|r| r.unwrap())
        .collect()
}
```

## 监控与测试

### 性能监控

#### Prometheus 指标

```rust
use prometheus::{Counter, Histogram, Registry};

lazy_static! {
    static ref HTTP_REQUESTS: Counter = Counter::new(
        "http_requests_total",
        "Total HTTP requests"
    ).unwrap();
    
    static ref HTTP_DURATION: Histogram = Histogram::new(
        "http_request_duration_seconds",
        "HTTP request duration"
    ).unwrap();
}

pub fn record_request(duration: f64) {
    HTTP_REQUESTS.inc();
    HTTP_DURATION.observe(duration);
}
```

#### 健康检查

```rust
#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: String,
    pub memory_mb: f64,
    pub uptime_seconds: u64,
}

pub async fn health_check(pool: &SqlitePool) -> HealthStatus {
    let db_status = match sqlx::query("SELECT 1").fetch_one(pool).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };
    
    HealthStatus {
        status: "ok".to_string(),
        database: db_status.to_string(),
        memory_mb: get_memory_usage(),
        uptime_seconds: get_uptime(),
    }
}
```

### 性能测试

#### 压力测试

使用 k6 进行压力测试：
```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '1m', target: 100 },
    { duration: '3m', target: 100 },
    { duration: '1m', target: 0 },
  ],
};

export default function () {
  let response = http.post('http://localhost:8080/api/v1/sessions', JSON.stringify({
    model_id: 'claude-3-opus',
    system_prompt: 'You are a helpful assistant',
  }), {
    headers: { 'Content-Type': 'application/json' },
  });
  
  check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });
  
  sleep(1);
}
```

#### 基准测试

Rust 基准测试：
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_vector_search(c: &mut Criterion) {
    let index = create_test_index();
    let query = vec![0.1; 768];
    
    c.bench_function("vector_search", |b| {
        b.iter(|| {
            index.search(black_box(&query), black_box(10))
        })
    });
}

criterion_group!(benches, benchmark_vector_search);
criterion_main!(benches);
```

## 性能优化检查清单

### 前端

- [ ] 启用代码分割
- [ ] 使用虚拟滚动处理长列表
- [ ] 优化状态管理（避免不必要的重渲染）
- [ ] 使用防抖/节流处理高频事件
- [ ] 启用生产构建优化
- [ ] 压缩静态资源
- [ ] 使用 Web Workers 处理计算密集任务

### 后端

- [ ] 数据库索引优化
- [ ] 连接池配置
- [ ] 启用查询缓存
- [ ] 使用批量操作
- [ ] 异步处理耗时任务
- [ ] 流式输出大数据
- [ ] 限流保护

### 向量检索

- [ ] 使用 HNSW 索引
- [ ] 批量检索优化
- [ ] 索引预热
- [ ] 内存映射文件

## 参考文档

- 后端架构：`docs/backend-architecture.md`
- API 规范：`docs/api-specification.md`
- 事件协议：`docs/event-protocol.md`
- 规范文件：`openspec/specs/*/spec.md`
