# 用户系统

https://miro.com/app/board/uXjVIvJOC3s=/

1. 识别的主要功能
   用户系统的核心功能包括用户的注册、登录、密码修改、权限管理、账户激活等。我们将围绕这些功能进行建模。

2. 事件风暴建模
   命令（Commands）：
   这些是系统请求的操作，它们触发事件。

Register User（注册用户）：用户提交注册请求，包括用户名、密码、电子邮箱等。

Login User（登录用户）：用户提交登录凭证（用户名、密码）进行登录。

Change Password（修改密码）：用户请求修改账户密码。

Update User Permissions（更新用户权限）：管理员修改用户的权限或角色。

Activate User Account（激活账户）：用户通过激活链接激活账户。

事件（Events）：
这些是系统中的状态变化，触发后会影响其他系统行为。

User Registered（用户注册成功）：用户成功注册，系统保存用户信息。

User Logged In（用户登录成功）：用户凭证有效，成功登录。

User Password Changed（用户密码修改成功）：用户修改了密码。

User Permissions Updated（用户权限更新）：管理员修改了用户的权限或角色。

User Account Activated（账户激活成功）：用户通过邮件激活链接激活账户。

聚合（Aggregates）：
聚合是业务规则的核心单元，负责处理命令并保持业务一致性。

User（用户聚合）：包含用户的基本信息，如用户名、邮箱、密码、角色、账户状态等。聚合会根据命令处理数据，并生成相应的事件。

策略（Policies）：
策略是基于某些事件触发的操作。

Send Activation Email（发送激活邮件）：当用户注册后，系统会发送激活邮件到用户邮箱。

Send Password Change Notification（发送密码修改通知）：当用户修改密码时，系统会发送通知邮件。

Send Permission Update Notification（发送权限更新通知）：当管理员更新用户权限时，系统会发送邮件通知用户。

外部系统（External Systems）：
外部系统是与系统集成的服务。

Email Service（邮件服务）：用来发送注册激活邮件、密码修改邮件、权限更新通知等。

3. 读模型（Read Models）
   读模型是为优化查询操作而设计的，它专注于数据读取，而不是业务逻辑处理。它们通常和写模型（命令和聚合）分开，以提高系统的性能和可扩展性。

User Details Read Model（用户详情查询模型）：提供快速查询用户信息，如用户名、邮箱、角色、账户状态等。

User Permissions Read Model（用户权限查询模型）：提供用户的权限和角色信息，通常用于权限验证和展示。

User Login History Read Model（用户登录历史查询模型）：记录用户的登录历史，用于审计和查询。

User Account Status Read Model（用户账户状态查询模型）：提供账户是否激活、是否被锁定等信息的查询。

4. 各个部分之间的关系
   命令触发事件：

Register User → 触发 User Registered 事件。

Login User → 触发 User Logged In 事件。

Change Password → 触发 User Password Changed 事件。

Update User Permissions → 触发 User Permissions Updated 事件。

事件触发聚合更新：

User Registered 事件触发 User 聚合，更新用户的状态。

User Logged In 事件更新 User 聚合，记录用户的登录状态。

事件触发策略：

User Registered → 触发 Send Activation Email 策略，发送激活邮件。

User Password Changed → 触发 Send Password Change Notification 策略，发送密码修改通知。

User Permissions Updated → 触发 Send Permission Update Notification 策略，通知用户权限已更新。

策略调用外部系统：

Send Activation Email → 调用外部系统 Email Service 发送激活邮件。

Send Password Change Notification → 调用外部系统 Email Service 发送密码修改通知。

事件触发读模型更新：

User Registered → 更新 User Details Read Model，保存用户的基本信息。

User Permissions Updated → 更新 User Permissions Read Model，保存用户的权限信息。

User Logged In → 更新 User Login History Read Model，记录用户的登录历史。

读模型提供查询服务：

通过 User Details Read Model 查询用户基本信息。

通过 User Permissions Read Model 查询用户权限。

通过 User Login History Read Model 查询用户登录历史。

通过 User Account Status Read Model 查询用户账户状态。

```
+------------------------------------+
|           Register User            |
|       (命令)                       |
+------------------------------------+
              |
              v
+------------------------------------+     +--------------------------------------+
|          User Registered           |---->|          User (聚合)               |
|           (事件)                   |     |  - 保存用户信息                   |
+------------------------------------+     +--------------------------------------+
              |                               |
              v                               v
+------------------------------------+     +--------------------------------------+
| Send Activation Email (策略)      |---->|  Email Service (外部系统)          |
+------------------------------------+     +--------------------------------------+
              |
              v
+------------------------------------+
|  User Details Read Model           |
|   (读模型)                         |
+------------------------------------+

+------------------------------------+
|           Login User               |
|        (命令)                      |
+------------------------------------+
              |
              v
+------------------------------------+     +--------------------------------------+
|          User Logged In            |---->|          User (聚合)               |
|           (事件)                   |     |  - 更新登录状态                   |
+------------------------------------+     +--------------------------------------+
              |
              v
+------------------------------------+
|  User Login History Read Model    |
|   (读模型)                         |
+------------------------------------+
```

```
user-system/
│
├── src/
│   ├── application/                # 应用层 (Application Layer)
│   │   ├── commands/               # 命令处理逻辑 (Command Handlers)
│   │   ├── queries/                # 查询处理逻辑 (Query Handlers)
│   │   └── services/               # 用例服务 (Application Services)
│   │   ├── events/                 # 事件处理逻辑 (Event Handlers)
│   │   └── listeners/              # 事件监听器 (Event Listeners) (可选，如果希望拆分)
│   │
│   ├── domain/                     # 领域层 (Domain Layer)
│   │   ├── user/                   # 用户领域 (User Domain)
│   │   │   ├── aggregates/         # 聚合根 (Aggregates)
│   │   │   ├── entities/           # 实体 (Entities)
│   │   │   ├── value_objects/      # 值对象 (Value Objects)
│   │   │   ├── events/             # 领域事件 (Domain Events)
│   │   │   ├── repositories/       # 仓储 (Repositories)
│   │   │   └── services/           # 领域服务 (Domain Services)
│   │   │
│   │   ├── password/               # 密码领域 (Password Domain)
│   │   │   ├── aggregates/         # 聚合根
│   │   │   ├── events/             # 领域事件
│   │   │   ├── services/           # 领域服务
│   │   │   └── value_objects/      # 值对象
│   │   │
│   │   ├── security/               # 安全领域 (Security Domain)
│   │   │   └── services/           # 领域服务 (例如：密码验证)
│   │   │
│   │   └── shared/                 # 共享领域 (Shared Domain)
│   │       ├── events/             # 共享领域事件
│   │       ├── exceptions/         # 共享异常处理
│   │       └── value_objects/      # 共享值对象
│   │
│   ├── infrastructure/             # 基础设施层 (Infrastructure Layer)
│   │   ├── external_systems/       # 外部系统接口 (例如：外部邮件服务、身份验证系统)
│   │   ├── persistence/            # 持久化 (数据库接口)
│   │   ├── repositories/           # 仓储实现
│   │   └── services/               # 基础设施服务（例如：HTTP 请求服务）
│   │
│   ├── interfaces/                 # 接口层 (Interface Layer)
│   │   ├── controllers/            # 控制器 (如：HTTP API 控制器)
│   │   ├── dtos/                   # 数据传输对象 (DTOs)
│   │   └── presenters/             # 展示层逻辑（如：响应格式化）
│   │
│   ├── config/                     # 配置文件
│   │   └── application_config.rs   # 配置类
│   │
│   └── main.rs                     # 程序入口
│
├── tests/                          # 测试文件
│   ├── domain/                     # 领域层单元测试
│   ├── application/                # 应用层单元测试
│   ├── infrastructure/             # 基础设施层单元测试
│   ├── interfaces/                 # 接口层单元测试
│   └── integration/                # 集成测试
│
├── Cargo.toml                      # 项目依赖文件 (Rust项目)
└── README.md                       # 项目说明文件

```
