# Change: 从 sz-admin 迁移用户权限管理基础模块

## Why
当前 bj-gover-llm 项目的 user-access 模块仅有骨架，缺少用户、角色、菜单、部门、权限等核心功能实现。sz-admin 是一个成熟的 RBAC 权限管理系统，基于 Spring Boot 3 + MyBatis Flex + Sa-Token，与本项目技术栈高度契合。通过迁移 sz-admin 的核心模块，可以快速建立完整的用户权限管理体系。

## What Changes

### 核心业务模块
- 迁移用户管理模块（User）到 modules/user-access
- 迁移角色管理模块（Role）到 modules/user-access
- 迁移菜单管理模块（Menu）到 modules/user-access
- 迁移部门管理模块（Dept）到 modules/user-access
- 迁移数据权限模块（DataRole）到 modules/user-access

### 基础设施代码（到 shared-kernel）
- 统一响应封装（ApiResult、PageResult）
- 全局异常处理（GlobalExceptionHandler、BaseException、BusinessException）
- 工具类库（StringUtils、DateUtils、JsonUtils、BeanCopyUtils、TreeUtils、PageUtils、AESUtil）
- 接口防抖注解（@Debounce）⭐
- 访问日志切面（AccessLogAspect）⭐
- 数据权限控制（SimpleDataScopeHelper）⭐

### 可选增强功能（后续迁移）
- Excel 导入导出（sz-common-excel）- 基于 EasyExcel 封装
- Redis 集成（sz-common-db-redis）- 封装常用操作和消息广播

### 技术栈配置
- 配置 MyBatis-Flex 1.9.5（替换当前注释的 1.7.7 版本）
- 配置 Sa-Token 1.38.0 权限认证框架
- 配置 Liquibase 数据库版本管理

### 数据库表结构（使用 Liquibase 管理）
- `sys_user` - 系统用户表（用户名、密码、手机号、昵称、性别、生日、头像、年龄、身份证、邮箱、账户状态、标签、最近登录时间）
- `sys_role` - 系统角色表（角色名称、权限标识、简介、是否锁定）
- `sys_menu` - 系统菜单表（路径、路由名称、标题、图标、组件路径、排序、层级、菜单类型、权限、是否隐藏、是否有子级、是否外链、是否全屏、是否固定、是否缓存、是否开启数据权限）
- `sys_dept` - 部门表（部门名称、父级ID、层级、排序、是否有子级、是否锁定）
- `sys_user_dept` - 用户-部门关系表
- `sys_user_role` - 用户-角色关联表
- `sys_user_data_role` - 用户-数据角色关联表
- `sys_role_menu` - 角色-菜单关联表

所有表均支持 MySQL 和达梦数据库，使用 Liquibase 进行版本管理和自动迁移。

## Impact
- Affected specs: user-access（新增完整实现）
- Affected code: 
  - `bj-gover-llm-service/shared-kernel/` - 新增基础设施代码
  - `bj-gover-llm-service/modules/user-access/` - 从空模块变为完整实现
  - `bj-gover-llm-service/main-server/pom.xml` - 新增 MyBatis-Flex 和 Sa-Token 依赖
  - `bj-gover-llm-service/main-server/src/main/resources/application.yml` - 新增配置
- Breaking changes: 无（这是新功能，不影响现有代码）

## Migration Strategy
采用分阶段迁移：
1. 第一阶段：基础设施（响应封装、异常处理、工具类、防抖注解、日志切面）
2. 第二阶段：核心实体和数据访问层（User、Role、Menu、Dept）
3. 第三阶段：业务服务层和控制器
4. 第四阶段：数据权限和安全配置（Sa-Token + 数据权限过滤）
5. 第五阶段：测试和验证
6. 第六阶段（可选）：Excel 导入导出、Redis 集成

## Technical Considerations
- sz-admin 使用 MyBatis Flex，需要配置正确版本（3.x）
- sz-admin 使用 Sa-Token 做权限认证，需要集成到 Spring Security 体系
- 需要适配达梦数据库的 SQL 方言
- 需要保持 DDD 架构风格，将 sz-admin 的三层架构代码重构为六边形架构
