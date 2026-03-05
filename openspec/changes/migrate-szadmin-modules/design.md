# 技术设计文档

## sz-admin 优秀设计分析

### 1. 接口防抖机制 ⭐⭐⭐⭐⭐

**设计亮点**：
- 通过注解实现接口防抖，防止短时间内重复请求
- 可自定义防抖时间
- 基于 Redis 存储防抖状态，支持分布式部署

**使用示例**：
```java
@Debounce(time = 1000)  // 1秒内防止重复提交
@PostMapping("/create")
public ApiResult<?> create(@RequestBody UserDTO dto) {
    return ApiResult.success(userService.create(dto));
}
```

**实现原理**：
- 使用 AOP 拦截带有 @Debounce 注解的方法
- 生成唯一键：userId + 方法签名 + 参数哈希
- 在 Redis 中设置过期时间
- 如果键存在则拒绝请求

### 2. 访问日志切面 ⭐⭐⭐⭐⭐

**设计亮点**：
- 自动记录所有 HTTP 请求和响应
- 包含请求参数、响应结果、耗时、IP 地址、用户 ID
- 慢查询自动告警（超过 2 秒）
- 支持白名单配置，跳过不需要记录的接口
- 支持敏感信息脱敏

**日志格式**：
```json
{
  "traceId": "xxx",
  "url": "/api/users/list",
  "method": "POST",
  "ip": "192.168.1.100",
  "userId": "1001",
  "param": {},
  "body": "{}",
  "requestBody": [],
  "timestamp": 1234567890,
  "ms": 150
}
```

### 3. 数据权限控制 ⭐⭐⭐⭐⭐

**设计亮点**：
- 基于 ThreadLocal 的数据权限过滤
- 支持部门级数据权限（本部门、本部门及子部门、全部）
- 使用 TransmittableThreadLocal 支持异步场景
- 与 MyBatis-Flex 集成，自动在 SQL 中添加数据权限过滤条件

**使用示例**：
```java
// 启用数据权限
SimpleDataScopeHelper.start(User.class);
try {
    List<User> users = userService.list();  // 自动过滤数据
} finally {
    SimpleDataScopeHelper.clearDataScope();
}
```

### 4. Excel 导入导出 ⭐⭐⭐⭐

**设计亮点**：
- 基于 EasyExcel 封装，性能优秀
- 支持字典值转换（如：1 → "男"，0 → "女"）
- 支持动态下拉选择
- 支持单元格合并策略
- 支持自定义样式和列宽

**使用示例**：
```java
// 导出
ExcelUtils.export(response, "用户列表", UserVO.class, userList);

// 导入
ExcelUtils.importExcel(file, UserDTO.class, new DefaultExcelListener<>());
```

### 5. Redis 集成 ⭐⭐⭐⭐

**设计亮点**：
- 封装常用 Redis 操作（get、set、delete、expire 等）
- 支持 WebSocket 消息通过 Redis 广播（多实例部署）
- 支持权限变更实时通知（用户权限变更后，通知所有实例刷新缓存）
- 支持分布式锁

## 架构适配策略

### sz-admin 原始架构（三层架构）
```
sz-service-admin/
├── controller/     # 控制器层
├── service/        # 服务层
├── mapper/         # 数据访问层
└── pojo/
    ├── dto/        # 数据传输对象
    └── vo/         # 视图对象
```

### bj-gover-llm 目标架构（六边形架构 + DDD）
```
modules/user-access/
├── domain/                    # 领域层
│   ├── model/                # 领域模型（实体、值对象）
│   ├── repository/           # 仓储接口
│   └── service/              # 领域服务
├── application/              # 应用层
│   ├── command/              # 命令对象
│   ├── query/                # 查询对象
│   └── service/              # 应用服务
├── infrastructure/           # 基础设施层
│   ├── persistence/          # 持久化实现
│   │   ├── entity/          # 数据库实体
│   │   ├── mapper/          # MyBatis Mapper
│   │   └── repository/      # 仓储实现
│   └── config/              # 配置
└── interfaces/               # 接口层
    ├── rest/                # REST 控制器
    ├── dto/                 # DTO
    └── assembler/           # 对象转换器
```

## 迁移映射关系

### 1. Controller → interfaces/rest
- sz-admin: `SysUserController.java`
- bj-gover-llm: `interfaces/rest/UserController.java`
- 职责：HTTP 请求处理、参数验证、响应封装

### 2. Service → application/service
- sz-admin: `SysUserService.java`
- bj-gover-llm: `application/service/UserApplicationService.java`
- 职责：业务流程编排、事务管理、权限校验

### 3. Mapper → infrastructure/persistence/mapper
- sz-admin: `SysUserMapper.java`
- bj-gover-llm: `infrastructure/persistence/mapper/UserMapper.java`
- 职责：SQL 映射、数据库操作

### 4. Entity → 分离为两层
- sz-admin: `SysUser.java`（单一实体）
- bj-gover-llm:
  - `domain/model/User.java`（领域模型，富含业务逻辑）
  - `infrastructure/persistence/entity/UserEntity.java`（数据库映射）

### 5. DTO/VO → interfaces/dto
- sz-admin: `pojo/dto/SysUserDTO.java`、`pojo/vo/SysUserVO.java`
- bj-gover-llm: `interfaces/dto/UserDTO.java`、`interfaces/dto/UserVO.java`
- 职责：数据传输、视图展示

## 技术栈适配

### MyBatis-Flex 配置
```yaml
# application.yml
mybatis-flex:
  mapper-locations: classpath*:/mapper/**/*.xml
  type-aliases-package: com.bjgov.*.infrastructure.persistence.entity
  configuration:
    map-underscore-to-camel-case: true
    log-impl: org.apache.ibatis.logging.slf4j.Slf4jImpl
```

### Sa-Token 配置
```yaml
# application.yml
sa-token:
  token-name: Authorization
  timeout: 2592000  # 30天
  activity-timeout: -1
  is-concurrent: true
  is-share: false
  token-style: uuid
  is-log: false
```

### 数据库方言适配
```java
// shared-kernel/persistence/dialect/
public interface SqlDialect {
    String buildPageQuery(String sql, int offset, int limit);
}

// MySQL 方言
public class MySqlDialect implements SqlDialect {
    @Override
    public String buildPageQuery(String sql, int offset, int limit) {
        return sql + " LIMIT " + offset + ", " + limit;
    }
}

// 达梦数据库方言
public class DamengDialect implements SqlDialect {
    @Override
    public String buildPageQuery(String sql, int offset, int limit) {
        return "SELECT * FROM (" + sql + ") LIMIT " + offset + ", " + limit;
    }
}
```

## 代码重构示例

### 原始代码（sz-admin）
```java
// SysUserController.java
@RestController
@RequestMapping("/admin/user")
public class SysUserController {
    @Autowired
    private SysUserService userService;
    
    @PostMapping("/list")
    public ApiResult<PageResult<SysUserVO>> list(@RequestBody SysUserDTO dto) {
        return ApiResult.success(userService.page(dto));
    }
}

// SysUserService.java
@Service
public class SysUserServiceImpl implements SysUserService {
    @Autowired
    private SysUserMapper userMapper;
    
    @Override
    public PageResult<SysUserVO> page(SysUserDTO dto) {
        Page<SysUser> page = userMapper.selectPage(dto);
        return PageResult.of(page, SysUserVO.class);
    }
}
```

### 重构后代码（bj-gover-llm）
```java
// interfaces/rest/UserController.java
@RestController
@RequestMapping("/api/users")
public class UserController {
    private final UserApplicationService userService;
    
    @PostMapping("/list")
    public ApiResult<PageResult<UserVO>> list(@RequestBody UserQueryCommand command) {
        return ApiResult.success(userService.queryUsers(command));
    }
}

// application/service/UserApplicationService.java
@Service
public class UserApplicationService {
    private final UserRepository userRepository;
    private final UserAssembler userAssembler;
    
    @Transactional(readOnly = true)
    public PageResult<UserVO> queryUsers(UserQueryCommand command) {
        PagedQuery<User> pagedQuery = userRepository.findByCondition(command);
        return userAssembler.toPagedVO(pagedQuery);
    }
}

// domain/repository/UserRepository.java
public interface UserRepository {
    PagedQuery<User> findByCondition(UserQueryCommand command);
    User findById(Long id);
    void save(User user);
    void delete(Long id);
}

// infrastructure/persistence/repository/UserRepositoryImpl.java
@Repository
public class UserRepositoryImpl implements UserRepository {
    private final UserMapper userMapper;
    private final UserAssembler userAssembler;
    
    @Override
    public PagedQuery<User> findByCondition(UserQueryCommand command) {
        Page<UserEntity> page = userMapper.selectPage(command);
        List<User> users = page.getRecords().stream()
            .map(userAssembler::toDomain)
            .collect(Collectors.toList());
        return new PagedQuery<>(users, page.getTotal());
    }
}
```

## 依赖管理

### pom.xml 配置
```xml
<!-- MyBatis-Flex -->
<dependency>
    <groupId>com.mybatis-flex</groupId>
    <artifactId>mybatis-flex-spring-boot-starter</artifactId>
    <version>1.9.5</version>
</dependency>

<!-- Sa-Token -->
<dependency>
    <groupId>cn.dev33</groupId>
    <artifactId>sa-token-spring-boot3-starter</artifactId>
    <version>1.38.0</version>
</dependency>

<!-- 达梦数据库驱动 -->
<dependency>
    <groupId>com.dameng</groupId>
    <artifactId>DmJdbcDriver18</artifactId>
    <version>8.1.2.192</version>
</dependency>
```

## 数据库表设计

### 用户表（sys_user）
```sql
CREATE TABLE sys_user (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    password VARCHAR(100) NOT NULL COMMENT '密码',
    nickname VARCHAR(50) COMMENT '昵称',
    email VARCHAR(100) COMMENT '邮箱',
    phone VARCHAR(20) COMMENT '手机号',
    avatar VARCHAR(255) COMMENT '头像',
    dept_id BIGINT COMMENT '部门ID',
    status TINYINT DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    create_by BIGINT,
    update_by BIGINT,
    deleted TINYINT DEFAULT 0 COMMENT '删除标记：0-未删除，1-已删除',
    INDEX idx_username (username),
    INDEX idx_dept_id (dept_id)
) COMMENT='用户表';
```

### 角色表（sys_role）
```sql
CREATE TABLE sys_role (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    role_name VARCHAR(50) NOT NULL COMMENT '角色名称',
    role_code VARCHAR(50) NOT NULL UNIQUE COMMENT '角色编码',
    description VARCHAR(255) COMMENT '描述',
    sort_num INT DEFAULT 0 COMMENT '排序',
    status TINYINT DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    create_by BIGINT,
    update_by BIGINT,
    deleted TINYINT DEFAULT 0,
    INDEX idx_role_code (role_code)
) COMMENT='角色表';
```

### 用户角色关联表（sys_user_role）
```sql
CREATE TABLE sys_user_role (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL COMMENT '用户ID',
    role_id BIGINT NOT NULL COMMENT '角色ID',
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE KEY uk_user_role (user_id, role_id),
    INDEX idx_user_id (user_id),
    INDEX idx_role_id (role_id)
) COMMENT='用户角色关联表';
```

## 迁移注意事项

1. **包名调整**：sz-admin 使用 `com.sz`，需要改为 `com.bjgov`
2. **类名调整**：去掉 `Sys` 前缀，如 `SysUser` → `User`
3. **注解调整**：保持 MyBatis-Flex 和 Sa-Token 注解不变
4. **配置调整**：适配项目的配置文件结构
5. **测试覆盖**：每个模块迁移后都要编写单元测试和集成测试
6. **仓库地址**：https://github.com/feiyuchuixue/sz-admin