# Change: 统一管理端与门户端为单一入口

## Why
当前管理端与门户端为两个独立 SPA 与登录入口，用户需要区分入口，且多角色用户体验割裂。统一入口并在登录后进行角色选择，可减少学习成本并保证一致体验。

## What Changes
- 统一为单一登录入口与统一会话管理
- 登录后若用户拥有多个角色，进入角色选择页并基于角色进入对应功能域
- 前端路由与菜单基于角色权限进行渲染与访问控制
- **BREAKING**：原管理端/门户端独立登录入口与部署入口将被替换

## Impact
- Affected specs: `user-access`（新增）
- Affected code: `bj-gover-llm-admin`、`bj-gover-llm-portal`、登录/鉴权/路由守卫、构建与部署配置
