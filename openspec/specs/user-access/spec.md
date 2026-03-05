# user-access Specification

## Purpose
TBD - created by archiving change add-unified-entry. Update Purpose after archive.
## Requirements
### Requirement: Unified Login Entry
系统 MUST 提供一个统一登录入口，适用于管理端与门户端用户。

#### Scenario: 单角色登录成功
- **WHEN** 用户通过统一入口登录且仅有一个角色
- **THEN** 系统 MUST 直接进入该角色的默认主页

#### Scenario: 多角色需要选择
- **WHEN** 用户通过统一入口登录且拥有多个角色
- **THEN** 系统 MUST 展示角色选择页并等待用户选择

#### Scenario: 无角色或未授权
- **WHEN** 用户登录后没有任何可用角色
- **THEN** 系统 MUST 提示无权限并阻止进入功能页面

### Requirement: Role Selection
系统 MUST 在用户拥有多个角色时允许其选择进入的角色，并建立角色上下文。

#### Scenario: 角色选择后进入
- **WHEN** 用户在角色选择页选择某角色
- **THEN** 系统 MUST 进入该角色默认主页并在会话中保存角色上下文

#### Scenario: 刷新保留角色
- **WHEN** 用户刷新页面
- **THEN** 系统 MUST 在会话有效期内恢复先前选择的角色上下文

### Requirement: Role-Based Navigation
系统 MUST 基于角色权限控制路由访问与菜单可见性。

#### Scenario: 菜单可见性
- **WHEN** 用户进入应用
- **THEN** 系统 MUST 仅显示该角色可访问的菜单与页面入口

#### Scenario: 未授权路由访问
- **WHEN** 用户访问未授权路由
- **THEN** 系统 MUST 阻止访问并返回未授权提示页

