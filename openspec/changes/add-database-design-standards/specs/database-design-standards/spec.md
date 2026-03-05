# Database Design Standards

数据库设计规范，定义表结构、字段、索引、关系等设计标准。

## ADDED Requirements

### Requirement: 表命名规范

数据库表名 MUST 遵循以下命名规范：
- 使用大写字母和下划线分隔
- 使用中英文双语命名格式：`ENGLISH_NAME_中文名`
- 英文部分使用单数形式（如 `USER` 而非 `USERS`）
- 中间表使用两个实体名组合（如 `SYS_USER_ROLE_用户角色`）

#### Scenario: 创建新表时命名

- **WHEN** 创建用户表
- **THEN** 表名为 `SYS_USER_用户`

#### Scenario: 创建关联表时命名

- **WHEN** 创建用户和角色的多对多关联表
- **THEN** 表名为 `SYS_USER_ROLE_用户角色`

### Requirement: 字段命名规范

表字段 MUST 遵循以下命名规范：
- 使用小写字母和下划线分隔
- 使用中英文双语命名格式：`english_name_中文名`
- 主键统一命名为 `id_主键`
- 外键命名格式：`{关联表英文名}_id_{中文说明}` （如 `org_id_所属组织ID`）
- 状态/类型字段统一使用 `_code` 后缀（如 `status_code_状态码`）
- 时间字段使用 `_time` 或 `_at` 后缀（如 `create_time_创建时间`）

#### Scenario: 定义主键字段

- **WHEN** 创建表的主键
- **THEN** 字段名为 `id_主键`，类型为 `bigint`

#### Scenario: 定义外键字段

- **WHEN** 引用组织表的外键
- **THEN** 字段名为 `org_id_所属组织ID`，类型为 `bigint`

#### Scenario: 定义状态字段

- **WHEN** 定义记录状态
- **THEN** 字段名为 `status_code_状态码`，类型为 `varchar`

### Requirement: 数据类型标准

字段数据类型 MUST 遵循以下标准：
- 主键和外键：`bigint`
- 状态码/枚举值：`varchar` 存储编码值（如 `ACTIVE`、`PENDING`）
- 布尔值：`boolean` 或 `tinyint(1)`
- 时间戳：`timestamp` 或 `datetime`
- 金额/精确数值：`numeric` 或 `decimal`
- 计数器：`int` 或 `bigint`
- 短文本：`varchar(长度)`，长度根据实际需求定义
- 长文本：`text`
- JSON 数据：`text` 或 `json`（根据数据库支持）

#### Scenario: 定义状态字段类型

- **WHEN** 定义状态字段
- **THEN** 使用 `varchar` 类型存储状态码（如 `ACTIVE`、`INACTIVE`、`PENDING`）

#### Scenario: 定义布尔字段类型

- **WHEN** 定义是否启用字段
- **THEN** 使用 `boolean` 类型，字段名如 `is_active_是否启用`

#### Scenario: 定义金额字段类型

- **WHEN** 定义金额或精确数值
- **THEN** 使用 `numeric` 或 `decimal` 类型，指定精度（如 `numeric(10,2)`）

### Requirement: 外键关系规范

表之间的外键关系 MUST 遵循以下规范：
- 所有外键字段 MUST 在 ER 图中明确标注 `FK`
- 外键关系 MUST 在 ER 图中使用关系线连接
- 关系基数 MUST 明确标注（一对一 `||--||`、一对多 `||--o{`、多对多通过中间表）
- 关系名称 MUST 使用中文描述业务含义
- 引用不存在的实体时 MUST 在同一 ER 图中定义该实体或添加注释说明

#### Scenario: 定义一对多关系

- **WHEN** 组织表和用户表是一对多关系
- **THEN** ER 图中标注 `SYS_ORGANIZATION_组织 ||--o{ SYS_USER_用户 : 一对多_所属`

#### Scenario: 定义多对多关系

- **WHEN** 用户和角色是多对多关系
- **THEN** 创建中间表 `SYS_USER_ROLE_用户角色`，包含 `user_id_用户ID` 和 `role_id_角色ID` 两个外键

#### Scenario: 跨 ER 图引用实体

- **WHEN** 在资源申请 ER 图中引用资源管理 ER 图的模型实体
- **THEN** 在资源申请 ER 图中定义简化版模型实体（仅包含必要字段）并添加注释说明完整定义位置

### Requirement: 审计字段标准

所有业务表 MUST 包含以下审计字段：
- `created_by_创建人` (`bigint`): 创建记录的用户 ID
- `create_time_创建时间` (`timestamp`): 记录创建时间
- `updated_by_更新人` (`bigint`): 最后更新记录的用户 ID（可选）
- `update_time_更新时间` (`timestamp`): 记录最后更新时间（可选）

流程相关表可使用以下字段替代：
- `create_id_创建人ID` + `create_by_创建人名称`
- `last_update_by_上次更新人` + `last_update_time_上次更新时间`

#### Scenario: 创建业务表时添加审计字段

- **WHEN** 创建新的业务表
- **THEN** 必须包含 `created_by_创建人`、`create_time_创建时间`、`updated_by_更新人`、`update_time_更新时间` 四个字段

#### Scenario: 流程表使用特殊审计字段

- **WHEN** 创建流程相关表（如工作流实例）
- **THEN** 可使用 `create_id_创建人ID`、`create_by_创建人名称`、`last_update_by_上次更新人`、`last_update_time_上次更新时间`

### Requirement: 索引命名规范

索引 MUST 遵循以下命名规范：
- 主键索引：`PK` 标注
- 唯一索引：`UK` 标注，字段名后添加
- 普通索引：`IDX_{表名}_{字段名}` 或在 ER 图中省略
- 外键索引：`FK_{表名}_{字段名}` 或在 ER 图中省略

#### Scenario: 标注主键索引

- **WHEN** 定义主键字段
- **THEN** 在字段后添加 `PK` 标注（如 `id_主键 PK`）

#### Scenario: 标注唯一索引

- **WHEN** 定义唯一约束字段
- **THEN** 在字段后添加 `UK` 标注（如 `username_登录账号 UK`）

#### Scenario: 标注外键

- **WHEN** 定义外键字段
- **THEN** 在字段后添加 `FK` 标注（如 `org_id_所属组织ID FK`）

### Requirement: 软删除标准

需要软删除的表 SHOULD 包含以下字段：
- `is_deleted_是否删除` (`boolean` 或 `tinyint(1)`): 删除标记，默认 `false` 或 `0`
- `deleted_by_删除人` (`bigint`): 删除操作的用户 ID（可选）
- `deleted_at_删除时间` (`timestamp`): 删除时间（可选）

#### Scenario: 需要软删除的表添加删除标记

- **WHEN** 用户表需要支持软删除
- **THEN** 添加 `is_deleted_是否删除` 字段，类型为 `boolean`，默认值为 `false`

#### Scenario: 记录删除操作审计信息

- **WHEN** 需要追踪谁在何时删除了记录
- **THEN** 添加 `deleted_by_删除人` 和 `deleted_at_删除时间` 字段

### Requirement: 版本控制字段

需要乐观锁的表 SHOULD 包含版本号字段：
- `version_版本号` (`int`): 记录版本号，每次更新递增

#### Scenario: 高并发更新场景使用版本号

- **WHEN** 资源配额表需要防止并发更新冲突
- **THEN** 添加 `version_版本号` 字段，类型为 `int`，默认值为 `0`

### Requirement: ER 图绘制规范

ER 图 MUST 遵循以下绘制规范：
- 使用 Mermaid erDiagram 语法
- 文件命名：`erd_{模块名}.mmd`
- 关系定义在实体定义之前
- 实体名使用 `ENGLISH_NAME_中文名` 格式
- 字段定义格式：`类型 字段名_中文名 [PK|UK|FK]`
- 关系描述使用中文（如 `一对多_所属`）
- 每个 ER 图聚焦单一业务域，避免过度复杂
- 跨域引用的实体需在当前图中定义简化版本

#### Scenario: 创建新的 ER 图文件

- **WHEN** 为用户权限模块创建 ER 图
- **THEN** 文件名为 `erd_user_access.mmd`，使用 Mermaid erDiagram 语法

#### Scenario: 定义实体关系

- **WHEN** 定义组织和用户的一对多关系
- **THEN** 使用 `SYS_ORGANIZATION_组织 ||--o{ SYS_USER_用户 : 一对多_所属`

#### Scenario: 定义实体字段

- **WHEN** 定义用户表的主键
- **THEN** 使用 `bigint id_主键 PK` 格式

### Requirement: 字段注释规范

所有字段 MUST 包含清晰的中文说明：
- 字段名本身包含中文说明（如 `username_登录账号`）
- 状态码字段 SHOULD 在注释或文档中说明可选值
- 枚举字段 SHOULD 在注释或文档中说明所有可能的值

#### Scenario: 定义状态码字段

- **WHEN** 定义申请单状态字段
- **THEN** 字段名为 `status_code_申请状态`，并在文档中说明可选值（如 `DRAFT`、`PENDING`、`APPROVED`、`REJECTED`）

#### Scenario: 定义枚举字段

- **WHEN** 定义环境类型字段
- **THEN** 字段名为 `env_type_code_环境类型`，并在文档中说明可选值（如 `DEV`、`TEST`、`PROD`）

### Requirement: 业务主键规范

除自增主键外，业务表 SHOULD 包含业务主键：
- 使用 `{业务前缀}_code_{中文说明}` 格式（如 `app_code_申请单编码`）
- 业务主键 MUST 添加唯一索引 `UK`
- 业务主键用于外部系统集成和业务查询

#### Scenario: 定义申请单业务主键

- **WHEN** 创建资源申请表
- **THEN** 添加 `app_code_申请单编码` 字段，类型为 `varchar`，标注 `UK`

#### Scenario: 定义系统编码业务主键

- **WHEN** 创建系统目录表
- **THEN** 添加 `system_code_系统编码` 字段，类型为 `varchar`，标注 `UK`

### Requirement: 租户隔离字段

多租户系统的表 SHOULD 包含租户标识：
- `tenant_id_租户ID` (`varchar`): 租户标识符
- 租户字段 SHOULD 作为复合索引的第一列

#### Scenario: 多租户流程表添加租户字段

- **WHEN** 创建流程实例表
- **THEN** 添加 `tenant_id_租户ID` 字段，类型为 `varchar`

### Requirement: JSON 字段使用规范

JSON 字段 SHOULD 谨慎使用，遵循以下原则：
- 仅用于非结构化或高度动态的数据（如扩展属性、配置参数）
- 字段名使用 `_json` 后缀或明确说明（如 `variable_变量JSON`、`ext_扩展JSON`）
- 不应在 JSON 字段中存储关键业务数据
- 需要查询的数据 SHOULD 提取为独立字段

#### Scenario: 存储流程变量使用 JSON

- **WHEN** 流程实例需要存储动态变量
- **THEN** 使用 `variable_变量JSON` 字段，类型为 `text`

#### Scenario: 存储扩展属性使用 JSON

- **WHEN** 任务参与者需要存储扩展信息
- **THEN** 使用 `ext_扩展JSON` 字段，类型为 `text`
