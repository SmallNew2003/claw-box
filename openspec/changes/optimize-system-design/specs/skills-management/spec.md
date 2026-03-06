## ADDED Requirements

### Requirement: 技能权限系统
系统 SHALL 支持技能权限声明和检查。

#### Scenario: 技能声明权限
- **WHEN** 技能安装时
- **THEN** 系统读取技能的权限声明
- **AND** 展示给用户确认

#### Scenario: 执行时权限检查
- **WHEN** 技能执行时
- **THEN** 系统检查技能声明的权限是否已授予
- **AND** 如果权限不足，返回 PERMISSION_DENIED 错误

### Requirement: 技能沙箱隔离
系统 SHALL 在隔离环境中执行技能。

#### Scenario: 资源限制
- **WHEN** 技能执行时
- **THEN** 系统限制执行超时（默认 30 秒）
- **AND** 系统限制内存使用（默认 100MB）

#### Scenario: 超时终止
- **WHEN** 技能执行超过超时时间
- **THEN** 系统终止技能执行
- **AND** 返回 SKILL_EXECUTION_TIMEOUT 错误
