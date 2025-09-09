# 概览

租户用户系统

- 用户管理
- 登录

## 数据库

- [sql 文件维护](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [sea-orm-cli 模型维护](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/)

## 待优化

事件流改成 event_outbox

## 生成实体

```bash
sea-orm-cli generate entity -o user-system/src/infrastructure/entitiy
```

房源模块

```bash
# 生成数据库
sqlx migrate add community -r --source domus/migrations
# 运行迁移
sqlx migrate run --source domus/migrations --database-url mysql://root:123456@localhost/domus
sqlx migrate run --source migrations --database-url mysql://root:123456@localhost/user_system
# 生成实体
sea-orm-cli generate entity -o domus/src/infrastructure/entitiy --database-url mysql://root:123456@localhost/domus

# 生成实体
sea-orm-cli generate entity -o tables --database-url mysql://root:123456@localhost/domus
```

初始化用户系统

```bash
cargo run --bin user_system -- init-system
cargo run --bin domus -- init-system
```
