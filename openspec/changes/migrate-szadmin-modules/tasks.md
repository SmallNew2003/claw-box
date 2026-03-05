# 迁移任务清单

## 1. 准备工作
- [x] 1.1 分析 sz-admin 项目结构，识别需要迁移的核心模块
- [x] 1.2 创建 user-access 模块的完整目录结构（domain、application、infrastructure、interfaces）
- [x] 1.3 配置 MyBatis-Flex 3.x 依赖（替换当前注释的版本）
- [x] 1.4 配置 Sa-Token 依赖

## 2. 基础设施层迁移

### 2.1 统一响应和异常处理
- [x] 2.1.1 迁移统一响应封装（ApiResult、PageResult、UploadResult）到 shared-kernel
- [x] 2.1.2 迁移全局异常处理（GlobalExceptionHandler）到 shared-kernel
- [x] 2.1.3 迁移异常体系（BaseException、BusinessException）到 shared-kernel
- [x] 2.1.4 迁移响应枚举（CommonResponseEnum、ResponseEnumTemplate）到 shared-kernel

### 2.2 工具类库
- [x] 2.2.1 迁移字符串工具（StringUtils）到 shared-kernel
- [x] 2.2.2 迁移日期工具（DateUtils）到 shared-kernel
- [x] 2.2.3 迁移 JSON 工具（JsonUtils）到 shared-kernel
- [x] 2.2.4 迁移对象拷贝工具（BeanCopyUtils）到 shared-kernel
- [x] 2.2.5 迁移树形结构工具（TreeUtils）到 shared-kernel
- [x] 2.2.6 迁移分页工具（PageUtils）到 shared-kernel
- [x] 2.2.7 迁移加密工具（AESUtil）到 shared-kernel
- [x] 2.2.8 迁移文件工具（FileUtils）到 shared-kernel
- [x] 2.2.9 迁移 HTTP 请求工具（HttpReqResUtil）到 shared-kernel
- [x] 2.2.10 迁移 Spring 上下文工具（SpringApplicationContextUtils）到 shared-kernel

### 2.3 接口防抖机制 ⭐
- [x] 2.3.1 迁移防抖注解（@Debounce、@DebounceIgnore）到 shared-kernel
- [x] 2.3.2 实现防抖切面（DebounceAspect）
- [x] 2.3.3 配置 Redis 存储防抖状态
- [x] 2.3.4 编写防抖功能测试

### 2.4 访问日志切面 ⭐
- [x] 2.4.1 迁移访问日志实体（AccessRequestLog、AccessResponseLog）到 shared-kernel
- [x] 2.4.2 迁移访问日志切面（AccessLogAspect）到 shared-kernel
- [x] 2.4.3 配置日志脱敏（DesensitizationUtil）
- [x] 2.4.4 配置慢查询告警阈值
- [x] 2.4.5 配置白名单（WhitelistProperties）

### 2.5 数据权限控制 ⭐
- [x] 2.5.1 迁移数据权限 ThreadLocal（SimpleDataScopeHelper）到 shared-kernel
- [x] 2.5.2 迁移数据权限控制器（ControlThreadLocal）到 shared-kernel
- [x] 2.5.3 配置 TransmittableThreadLocal 支持异步场景

### 2.6 包名适配
- [x] 2.6.1 将所有 com.sz 包名改为 com.bjgov.shared
- [x] 2.6.2 更新所有导入语句
- [x] 2.6.3 验证编译通过

## 3. 用户管理模块迁移
- [x] 3.1 创建用户领域模型（User 实体、值对象）
- [x] 3.2 迁移用户数据访问层（SysUserEntity、UserMapper、SysUserMapper）
- [x] 3.3 迁移用户仓储实现（UserRepository、UserRepositoryImpl）
- [x] 3.4 迁移用户服务层（SysUserService、SysUserServiceImpl）
- [x] 3.5 迁移用户控制器（SysUserController）
- [x] 3.6 迁移用户 DTO 和 VO（SysUserCreateDTO、SysUserUpdateDTO、SysUserListDTO、SysUserPasswordDTO、SysUserTagDTO、UserDeptDTO、SysUserVO）
- [x] 3.7 创建用户表 SQL 脚本（MySQL 和达梦数据库）

## 4. 角色管理模块迁移
- [ ] 4.1 创建角色领域模型（Role 实体）
- [ ] 4.2 迁移角色数据访问层（RoleEntity、RoleMapper）
- [ ] 4.3 迁移角色仓储实现（RoleRepository）
- [ ] 4.4 迁移角色服务层（RoleService）
- [x] 4.5 迁移角色控制器（RoleController）
- [ ] 4.6 迁移角色 DTO 和 VO（RoleDTO、RoleVO）
- [ ] 4.7 创建角色表 SQL 脚本

## 5. 菜单管理模块迁移
- [ ] 5.1 创建菜单领域模型（Menu 实体）
- [ ] 5.2 迁移菜单数据访问层（MenuEntity、MenuMapper）
- [ ] 5.3 迁移菜单仓储实现（MenuRepository）
- [ ] 5.4 迁移菜单服务层（MenuService）
- [ ] 5.5 迁移菜单控制器（MenuController）
- [ ] 5.6 迁移菜单 DTO 和 VO（MenuDTO、MenuVO）
- [ ] 5.7 创建菜单表 SQL 脚本

## 6. 部门管理模块迁移
- [ ] 6.1 创建部门领域模型（Dept 实体）
- [ ] 6.2 迁移部门数据访问层（DeptEntity、DeptMapper）
- [ ] 6.3 迁移部门仓储实现（DeptRepository）
- [ ] 6.4 迁移部门服务层（DeptService）
- [ ] 6.5 迁移部门控制器（DeptController）
- [ ] 6.6 迁移部门 DTO 和 VO（DeptDTO、DeptVO）
- [ ] 6.7 创建部门表 SQL 脚本

## 7. 数据权限模块迁移
- [ ] 7.1 创建数据权限领域模型（DataRole 实体）
- [ ] 7.2 迁移数据权限数据访问层（DataRoleEntity、DataRoleMapper）
- [ ] 7.3 迁移数据权限仓储实现（DataRoleRepository）
- [ ] 7.4 迁移数据权限服务层（DataRoleService）
- [ ] 7.5 迁移数据权限拦截器
- [ ] 7.6 创建数据权限表 SQL 脚本

## 8. 安全配置
- [ ] 8.1 配置 Sa-Token 认证拦截器
- [ ] 8.2 配置 Sa-Token 权限注解支持
- [ ] 8.3 实现登录认证接口
- [ ] 8.4 实现权限校验逻辑
- [ ] 8.5 配置跨域和安全头

## 9. 数据库配置
- [x] 9.1 配置 MyBatis-Flex 数据源
- [x] 9.2 配置 MySQL 方言
- [x] 9.3 配置达梦数据库方言
- [x] 9.4 创建数据库初始化脚本（Liquibase）
- [x] 9.5 添加测试数据脚本（初始化管理员账户）

## 10. 集成测试
- [ ] 10.1 测试用户 CRUD 接口
- [ ] 10.2 测试角色 CRUD 接口
- [ ] 10.3 测试菜单 CRUD 接口
- [ ] 10.4 测试部门 CRUD 接口
- [ ] 10.5 测试数据权限过滤
- [ ] 10.6 测试登录认证流程
- [ ] 10.7 测试权限校验

## 11. 文档更新
- [ ] 11.1 更新 API 文档（Knife4j）
- [ ] 11.2 更新数据库设计文档
- [ ] 11.3 更新部署文档
- [ ] 11.4 编写迁移说明文档

## 12. 可选增强功能（后续迁移）

### 12.1 Excel 导入导出
- [ ] 12.1.1 迁移 Excel 工具类（ExcelUtils）
- [ ] 12.1.2 迁移 Excel 监听器（ExcelListener、DefaultExcelListener）
- [ ] 12.1.3 迁移 Excel 注解（@DictFormat、@CellMerge）
- [ ] 12.1.4 迁移 Excel 转换器（字典值转换）
- [ ] 12.1.5 迁移 Excel 策略（样式、列宽、单元格合并）
- [ ] 12.1.6 测试 Excel 导入导出功能

### 12.2 Redis 集成
- [ ] 12.2.1 迁移 Redis 工具类（RedisUtils、RedisCache）
- [ ] 12.2.2 迁移 Redis 配置（RedisConfiguration）
- [ ] 12.2.3 迁移 Redis 消息监听器（WebSocket 消息广播）
- [ ] 12.2.4 迁移权限变更通知机制
- [ ] 12.2.5 配置 Redis 连接池
- [ ] 12.2.6 测试 Redis 功能

## 验证命令

```bash
# 编译检查
cd bj-gover-llm-service
mvn clean compile

# 运行测试
mvn test

# 启动应用
cd main-server
mvn spring-boot:run

# 访问 API 文档
open http://localhost:8080/api/doc.html
```
