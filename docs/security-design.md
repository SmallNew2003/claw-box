# Security Design - Tauri Security Configuration

<!-- 版本: v1.0.0 -->
<!-- 最后更新: 2026-03-06 -->
<!-- 维护者: 开发团队 -->

## 概述

Claw Box 使用 Tauri 框架构建桌面应用，本文档定义安全配置、密钥存储、权限控制和威胁防护策略。

### 安全目标

- 保护用户 API Token 和敏感配置
- 防止恶意代码注入和 XSS 攻击
- 限制前端对系统资源的访问
- 确保进程间通信（IPC）安全
- 审计关键操作和数据访问

### 威胁模型

潜在威胁：
- 恶意技能或 MCP 服务器
- 前端代码注入攻击
- 本地文件系统未授权访问
- API Token 泄露
- 中间人攻击（MITM）

## Tauri 安全架构

### 进程隔离

Tauri 采用多进程架构：

```
┌─────────────────────────────────────┐
│         Main Process (Rust)         │
│  - 系统 API 访问                     │
│  - 文件系统操作                      │
│  - 数据库访问                        │
│  - 网络请求                          │
└─────────────────────────────────────┘
              ↕ IPC (Tauri Command)
┌─────────────────────────────────────┐
│      Renderer Process (WebView)     │
│  - UI 渲染                           │
│  - 用户交互                          │
│  - 受限的系统访问                    │
└─────────────────────────────────────┘
```

隔离原则：
- 前端无法直接访问文件系统
- 前端无法直接执行系统命令
- 所有敏感操作通过 Tauri Command 调用
- Rust 后端验证所有前端请求

### CSP（Content Security Policy）

配置文件：`src-tauri/tauri.conf.json`

```json
{
  "tauri": {
    "security": {
      "csp": {
        "default-src": "'self'",
        "script-src": "'self' 'wasm-unsafe-eval'",
        "style-src": "'self' 'unsafe-inline'",
        "img-src": "'self' data: https:",
        "font-src": "'self' data:",
        "connect-src": "'self' http://localhost:8080 https://api.anthropic.com https://api.openai.com",
        "frame-src": "'none'",
        "object-src": "'none'",
        "base-uri": "'self'"
      }
    }
  }
}
```

CSP 规则说明：
- `default-src 'self'`: 默认只允许同源资源
- `script-src 'wasm-unsafe-eval'`: 允许 WASM 执行
- `style-src 'unsafe-inline'`: 允许内联样式（Tailwind CSS）
- `connect-src`: 限制网络请求目标
- `frame-src 'none'`: 禁止嵌入 iframe
- `object-src 'none'`: 禁止插件

### 允许列表（Allowlist）

限制前端可调用的 Tauri API：

```json
{
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "createDir": true,
        "removeFile": true,
        "scope": ["$APPDATA/*", "$APPCONFIG/*"]
      },
      "path": {
        "all": true
      },
      "dialog": {
        "all": false,
        "open": true,
        "save": true
      },
      "shell": {
        "all": false,
        "execute": false,
        "sidecar": false,
        "open": true
      },
      "http": {
        "all": false,
        "request": true,
        "scope": [
          "http://localhost:8080/*",
          "https://api.anthropic.com/*",
          "https://api.openai.com/*"
        ]
      },
      "notification": {
        "all": true
      },
      "clipboard": {
        "all": false,
        "writeText": true,
        "readText": false
      }
    }
  }
}
```

权限说明：
- 文件系统：仅限应用数据目录
- Shell：禁止执行任意命令
- HTTP：白名单限制请求目标
- 剪贴板：只允许写入，不允许读取

## 密钥存储

### 存储方案

使用操作系统原生密钥链：

| 操作系统 | 存储方案 |
|---------|---------|
| macOS | Keychain |
| Windows | Credential Manager |
| Linux | Secret Service API (libsecret) |

### Rust 实现

依赖：`keyring = "2.0"`

```rust
use keyring::Entry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiCredentials {
    pub provider: String,
    pub api_key: String,
}

pub struct SecureStorage {
    service_name: String,
}

impl SecureStorage {
    pub fn new() -> Self {
        Self {
            service_name: "ai.clawbox.credentials".to_string(),
        }
    }
    
    pub fn store_api_key(&self, provider: &str, api_key: &str) -> Result<(), String> {
        let entry = Entry::new(&self.service_name, provider)
            .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
        
        entry.set_password(api_key)
            .map_err(|e| format!("Failed to store API key: {}", e))?;
        
        Ok(())
    }
    
    pub fn retrieve_api_key(&self, provider: &str) -> Result<String, String> {
        let entry = Entry::new(&self.service_name, provider)
            .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
        
        entry.get_password()
            .map_err(|e| format!("Failed to retrieve API key: {}", e))
    }
    
    pub fn delete_api_key(&self, provider: &str) -> Result<(), String> {
        let entry = Entry::new(&self.service_name, provider)
            .map_err(|e| format!("Failed to create keyring entry: {}", e))?;
        
        entry.delete_password()
            .map_err(|e| format!("Failed to delete API key: {}", e))
    }
}
```

### Tauri Command

```rust
use tauri::State;

#[tauri::command]
async fn store_api_key(
    provider: String,
    api_key: String,
    storage: State<'_, SecureStorage>,
) -> Result<(), String> {
    storage.store_api_key(&provider, &api_key)
}

#[tauri::command]
async fn retrieve_api_key(
    provider: String,
    storage: State<'_, SecureStorage>,
) -> Result<String, String> {
    storage.retrieve_api_key(&provider)
}

#[tauri::command]
async fn delete_api_key(
    provider: String,
    storage: State<'_, SecureStorage>,
) -> Result<(), String> {
    storage.delete_api_key(&provider)
}
```

### 前端调用

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 存储 API Key
async function storeApiKey(provider: string, apiKey: string): Promise<void> {
  await invoke('store_api_key', { provider, apiKey });
}

// 获取 API Key
async function retrieveApiKey(provider: string): Promise<string> {
  return await invoke('retrieve_api_key', { provider });
}

// 删除 API Key
async function deleteApiKey(provider: string): Promise<void> {
  await invoke('delete_api_key', { provider });
}
```

## 权限控制

### 技能权限系统

技能执行需要声明权限：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    FileRead,
    FileWrite,
    Network,
    Execute,
    Clipboard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub permissions: Vec<Permission>,
}

pub struct PermissionChecker {
    granted_permissions: HashMap<String, Vec<Permission>>,
}

impl PermissionChecker {
    pub fn check(&self, skill_id: &str, permission: &Permission) -> bool {
        self.granted_permissions
            .get(skill_id)
            .map(|perms| perms.contains(permission))
            .unwrap_or(false)
    }
    
    pub fn grant(&mut self, skill_id: String, permissions: Vec<Permission>) {
        self.granted_permissions.insert(skill_id, permissions);
    }
    
    pub fn revoke(&mut self, skill_id: &str) {
        self.granted_permissions.remove(skill_id);
    }
}
```

## 网络安全

### HTTPS 强制

所有外部 API 调用使用 HTTPS：

```rust
use reqwest::Client;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .https_only(true)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client }
    }
}
```

## 审计日志

### 日志记录

记录所有敏感操作：

```rust
use tracing::{info, warn, error};

#[tauri::command]
async fn store_api_key(
    provider: String,
    api_key: String,
    storage: State<'_, SecureStorage>,
) -> Result<(), String> {
    info!("Storing API key for provider: {}", provider);
    
    match storage.store_api_key(&provider, &api_key) {
        Ok(_) => {
            info!("API key stored successfully for provider: {}", provider);
            Ok(())
        }
        Err(e) => {
            error!("Failed to store API key for provider {}: {}", provider, e);
            Err(e)
        }
    }
}
```

## 参考文档

- Tauri 安全指南：https://tauri.app/v1/guides/security/
- 后端架构：`docs/backend-architecture.md`
- API 规范：`docs/api-specification.md`
- 规范文件：`openspec/specs/*/spec.md`
