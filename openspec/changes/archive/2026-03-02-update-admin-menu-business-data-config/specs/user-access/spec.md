## MODIFIED Requirements
### Requirement: Role-Based Navigation
系统 MUST 基于角色权限控制路由访问与菜单可见性，并在管理端展示规范化的业务数据配置菜单。

#### Scenario: 菜单可见性
- **WHEN** 用户进入应用
- **THEN** 系统 MUST 仅显示该角色可访问的菜单与页面入口

#### Scenario: 业务数据配置菜单展示
- **WHEN** 具备相关权限的管理端用户进入应用
- **THEN** 系统 MUST 在菜单中展示“业务数据配置”，并包含“系统目录”“需求分类”“需求目录”子项

#### Scenario: 未授权路由访问
- **WHEN** 用户访问未授权路由
- **THEN** 系统 MUST 阻止访问并返回未授权提示页
