# OpenClaw 平台 MVP Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 构建独立前后端平台的 MVP：Agent Teams + 记忆系统 + Skills + MCP，默认单机一体化部署，提供 Web 与 Tauri 桌面端共用 API。

**Architecture:** Rust Core Service 统一业务 API（HTTP/JSON + SSE），本地 SQLite + 向量检索，Web/Tauri 作为客户端。通过 OpenClaw 兼容适配层读取技能与 MCP 配置。

**Tech Stack:** Rust (Axum, SQLx), SQLite, React + Vite, Tauri, TypeScript

---

### Task 1: 建立参考仓库目录（可选但推荐）

**Files:**
- Create: `references/README.md`

**Step 1: 写一个失败的检查用例（占位）**

```bash
# 仅占位：执行 ls 预期 references/ 不存在
ls references/
```

**Step 2: 运行以确认失败**

Run: `ls references/`
Expected: 失败，提示目录不存在

**Step 3: 创建参考目录与说明**

```bash
mkdir -p references
cat <<'DOC' > references/README.md
# Reference Repos

- https://github.com/openclaw/openclaw
- https://github.com/ruvnet/ruflo
- https://github.com/zeroclaw-labs/zeroclaw

说明：实现阶段可按需拉取或用 git submodule 引入。
DOC
```

**Step 4: 运行检查**

Run: `ls references/`
Expected: 输出 `README.md`

**Step 5: Commit**

```bash
git add references/README.md
git commit -m "feat(docs): 添加参考仓库说明"
```

### Task 2: 初始化 Rust Core Service 骨架与健康检查

**Files:**
- Create: `platform/Cargo.toml`
- Create: `platform/crates/core-service/Cargo.toml`
- Create: `platform/crates/core-service/src/main.rs`
- Create: `platform/crates/core-service/src/http/mod.rs`
- Test: `platform/crates/core-service/tests/health_check.rs`

**Step 1: 写失败测试**

```rust
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn health_check_returns_ok() {
    let app = core_service::http::router();
    let response = app
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}
```

**Step 2: 运行测试确认失败**

Run: `cd platform && cargo test -p core-service`
Expected: FAIL（找不到 core_service::http::router 或路由未实现）

**Step 3: 实现最小 HTTP 服务与 /health**

```rust
// main.rs
#[tokio::main]
async fn main() {
    let app = core_service::http::router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8787").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// http/mod.rs
pub fn router() -> axum::Router {
    axum::Router::new().route("/health", axum::routing::get(|| async { "ok" }))
}
```

**Step 4: 运行测试确认通过**

Run: `cd platform && cargo test -p core-service`
Expected: PASS

**Step 5: Commit**

```bash
git add platform/Cargo.toml platform/crates/core-service
 git commit -m "feat(core): 初始化核心服务与健康检查"
```

### Task 3: 添加 SQLite 存储层与基础迁移

**Files:**
- Create: `platform/crates/core-service/src/storage/mod.rs`
- Create: `platform/crates/core-service/src/storage/migrate.rs`
- Create: `platform/crates/core-service/migrations/0001_init.sql`
- Modify: `platform/crates/core-service/src/main.rs`
- Test: `platform/crates/core-service/tests/storage_smoke.rs`

**Step 1: 写失败测试**

```rust
#[tokio::test]
async fn storage_can_connect_and_migrate() {
    let pool = core_service::storage::init_sqlite("sqlite::memory:").await.unwrap();
    let rows: (i64,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();
    assert_eq!(rows.0, 1);
}
```

**Step 2: 运行测试确认失败**

Run: `cd platform && cargo test -p core-service storage_can_connect_and_migrate`
Expected: FAIL（storage 模块不存在）

**Step 3: 实现最小存储与迁移**

```rust
pub async fn init_sqlite(url: &str) -> anyhow::Result<sqlx::SqlitePool> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect(url).await?;
    sqlx::migrate!("migrations").run(&pool).await?;
    Ok(pool)
}
```

**Step 4: 运行测试确认通过**

Run: `cd platform && cargo test -p core-service storage_can_connect_and_migrate`
Expected: PASS

**Step 5: Commit**

```bash
git add platform/crates/core-service/src/storage platform/crates/core-service/migrations
 git commit -m "feat(storage): 初始化SQLite存储与迁移"
```

### Task 4: 定义核心领域模型（Teams / Runs / Memory / Skills / MCP）

**Files:**
- Create: `platform/crates/core-service/src/domain/mod.rs`
- Create: `platform/crates/core-service/src/domain/team.rs`
- Create: `platform/crates/core-service/src/domain/run.rs`
- Create: `platform/crates/core-service/src/domain/memory.rs`
- Create: `platform/crates/core-service/src/domain/skill.rs`
- Create: `platform/crates/core-service/src/domain/mcp.rs`
- Test: `platform/crates/core-service/tests/domain_serialization.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn team_can_serialize() {
    let team = core_service::domain::Team::new("core");
    let json = serde_json::to_string(&team).unwrap();
    assert!(json.contains("core"));
}
```

**Step 2: 运行测试确认失败**

Run: `cd platform && cargo test -p core-service team_can_serialize`
Expected: FAIL（Team 未定义）

**Step 3: 实现最小模型结构**

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team { pub id: String, pub name: String }
impl Team { pub fn new(name: &str) -> Self { Self { id: cuid::cuid2(), name: name.into() } } }
```

**Step 4: 运行测试确认通过**

Run: `cd platform && cargo test -p core-service team_can_serialize`
Expected: PASS

**Step 5: Commit**

```bash
git add platform/crates/core-service/src/domain
 git commit -m "feat(domain): 添加核心领域模型"
```

### Task 5: 提供 Teams 与 Runs 的基础 API

**Files:**
- Create: `platform/crates/core-service/src/http/teams.rs`
- Create: `platform/crates/core-service/src/http/runs.rs`
- Modify: `platform/crates/core-service/src/http/mod.rs`
- Test: `platform/crates/core-service/tests/http_teams.rs`

**Step 1: 写失败测试**

```rust
#[tokio::test]
async fn create_team_returns_201() {
    let app = core_service::http::router();
    let response = app
        .oneshot(Request::post("/api/teams")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"core"}"#)).unwrap())
        .await.unwrap();
    assert_eq!(response.status(), 201);
}
```

**Step 2: 运行测试确认失败**

Run: `cd platform && cargo test -p core-service create_team_returns_201`
Expected: FAIL（路由/处理器不存在）

**Step 3: 实现最小 API（内存存储占位）**

```rust
// POST /api/teams -> 201 + Team
// POST /api/runs -> 201 + Run (最小字段)
```

**Step 4: 运行测试确认通过**

Run: `cd platform && cargo test -p core-service create_team_returns_201`
Expected: PASS

**Step 5: Commit**

```bash
git add platform/crates/core-service/src/http
 git commit -m "feat(api): 添加Teams与Runs基础接口"
```

### Task 6: 记忆系统 API（最小检索）

**Files:**
- Create: `platform/crates/core-service/src/http/memory.rs`
- Modify: `platform/crates/core-service/src/http/mod.rs`
- Test: `platform/crates/core-service/tests/http_memory.rs`

**Step 1: 写失败测试**

```rust
#[tokio::test]
async fn create_memory_item_returns_201() {
    let app = core_service::http::router();
    let response = app
        .oneshot(Request::post("/api/memory")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"content":"hello"}"#)).unwrap())
        .await.unwrap();
    assert_eq!(response.status(), 201);
}
```

**Step 2: 运行测试确认失败**

Run: `cd platform && cargo test -p core-service create_memory_item_returns_201`
Expected: FAIL

**Step 3: 实现最小 API（本地 SQLite 写入 + 简单 LIKE 检索）**

```rust
// POST /api/memory -> 201
// GET /api/memory/search?q= -> 200
```

**Step 4: 运行测试确认通过**

Run: `cd platform && cargo test -p core-service create_memory_item_returns_201`
Expected: PASS

**Step 5: Commit**

```bash
git add platform/crates/core-service/src/http/memory.rs
 git commit -m "feat(memory): 添加记忆写入与检索接口"
```

### Task 7: Skills 与 MCP 基础管理 API

**Files:**
- Create: `platform/crates/core-service/src/http/skills.rs`
- Create: `platform/crates/core-service/src/http/mcp.rs`
- Modify: `platform/crates/core-service/src/http/mod.rs`
- Test: `platform/crates/core-service/tests/http_skills_mcp.rs`

**Step 1: 写失败测试**

```rust
#[tokio::test]
async fn create_skill_returns_201() {
    let app = core_service::http::router();
    let response = app
        .oneshot(Request::post("/api/skills")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"tool"}"#)).unwrap())
        .await.unwrap();
    assert_eq!(response.status(), 201);
}
```

**Step 2: 运行测试确认失败**

Run: `cd platform && cargo test -p core-service create_skill_returns_201`
Expected: FAIL

**Step 3: 实现最小管理接口**

```rust
// POST /api/skills
// POST /api/mcp/servers
// POST /api/mcp/servers/{id}/sync (占位：写入工具索引)
```

**Step 4: 运行测试确认通过**

Run: `cd platform && cargo test -p core-service create_skill_returns_201`
Expected: PASS

**Step 5: Commit**

```bash
git add platform/crates/core-service/src/http/skills.rs platform/crates/core-service/src/http/mcp.rs
 git commit -m "feat(skills): 添加Skills与MCP基础管理接口"
```

### Task 8: 事件流（SSE）与运行日志推送

**Files:**
- Create: `platform/crates/core-service/src/http/events.rs`
- Modify: `platform/crates/core-service/src/http/mod.rs`
- Test: `platform/crates/core-service/tests/http_events.rs`

**Step 1: 写失败测试**

```rust
#[tokio::test]
async fn events_endpoint_streams() {
    let app = core_service::http::router();
    let response = app
        .oneshot(Request::get("/api/events").body(Body::empty()).unwrap())
        .await.unwrap();
    assert_eq!(response.status(), 200);
}
```

**Step 2: 运行测试确认失败**

Run: `cd platform && cargo test -p core-service events_endpoint_streams`
Expected: FAIL

**Step 3: 实现 SSE（broadcast channel 占位）**

```rust
// GET /api/events -> SSE
```

**Step 4: 运行测试确认通过**

Run: `cd platform && cargo test -p core-service events_endpoint_streams`
Expected: PASS

**Step 5: Commit**

```bash
git add platform/crates/core-service/src/http/events.rs
 git commit -m "feat(events): 添加SSE事件流接口"
```

### Task 9: Web 前端脚手架与最小控制台页面

**Files:**
- Create: `apps/web/package.json`
- Create: `apps/web/src/main.tsx`
- Create: `apps/web/src/App.tsx`
- Create: `apps/web/src/api/client.ts`

**Step 1: 写失败检查（占位）**

```bash
test -f apps/web/src/App.tsx
```

**Step 2: 运行确认失败**

Run: `test -f apps/web/src/App.tsx`
Expected: 失败

**Step 3: 初始化 Vite + 基础页面**

```bash
npm create vite@latest apps/web -- --template react-ts
```

**Step 4: 运行检查**

Run: `test -f apps/web/src/App.tsx`
Expected: 成功

**Step 5: Commit**

```bash
git add apps/web
git commit -m "feat(web): 初始化Web控制台脚手架"
```

### Task 10: Tauri 桌面壳（mac 优先）

**Files:**
- Create: `apps/desktop/src-tauri/tauri.conf.json`
- Create: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/package.json`

**Step 1: 写失败检查（占位）**

```bash
test -f apps/desktop/src-tauri/tauri.conf.json
```

**Step 2: 运行确认失败**

Run: `test -f apps/desktop/src-tauri/tauri.conf.json`
Expected: 失败

**Step 3: 初始化 Tauri 并对接 Web 前端**

```bash
npm create tauri-app@latest apps/desktop
```

**Step 4: 运行检查**

Run: `test -f apps/desktop/src-tauri/tauri.conf.json`
Expected: 成功

**Step 5: Commit**

```bash
git add apps/desktop
git commit -m "feat(desktop): 初始化Tauri桌面壳"
```
