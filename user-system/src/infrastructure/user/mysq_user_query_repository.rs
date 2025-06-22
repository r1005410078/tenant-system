use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::user_query_repository::UserQueryRepository,
    infrastructure::{
        dtos::{
            user_login_history_dto::{self, UserLoginHistoryDto},
            user_query_dto::UserQueryDto,
        },
        entitiy::{self, user_login_history},
    },
};

pub struct MysqlUserQueryRepository {
    pool: Arc<DbConn>,
}

impl MysqlUserQueryRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserQueryRepository for MysqlUserQueryRepository {
    // 查询所以用户登录历史
    async fn get_user_login_history(
        &self,
        user_id: &str,
        table_data_request: &TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserLoginHistoryDto>> {
        let models = user_login_history::Entity::find()
            .filter(user_login_history::Column::UserId.eq(user_id))
            .order_by_desc(user_login_history::Column::LoginAt)
            .offset((table_data_request.page - 1) * table_data_request.page_size)
            .limit(table_data_request.page_size)
            .all(self.pool.as_ref())
            .await?;

        let total = user_login_history::Entity::find()
            .filter(user_login_history::Column::UserId.eq(user_id))
            .count(self.pool.as_ref())
            .await?;

        Ok(TableDataResponse {
            total,
            data: models.iter().map(|model| model.clone().into()).collect(),
        })
    }

    // 保持用户登录历史
    async fn save_user_login_history(
        &self,
        user_login_history: UserLoginHistoryDto,
    ) -> anyhow::Result<()> {
        let model = user_login_history::ActiveModel {
            user_id: Set(user_login_history.user_id),
            username: Set(user_login_history.username),
            status: Set(user_login_history.status),
            login_at: Set(user_login_history.login_at),
            ..Default::default()
        };

        user_login_history::Entity::insert(model)
            .exec(self.pool.as_ref())
            .await?;

        Ok(())
    }

    // 删除用户信息
    async fn delete_user(&self, user_id: &str) -> anyhow::Result<()> {
        entitiy::user_query::Entity::delete_many()
            .filter(entitiy::user_query::Column::UserId.eq(user_id))
            .exec(self.pool.as_ref())
            .await?;
        Ok(())
    }

    // 更新用户信息
    async fn update_user(&self, user: UserQueryDto) -> anyhow::Result<()> {
        let model = entitiy::user_query::ActiveModel {
            user_id: Set(user.user_id),
            username: Set(user.username),
            email: Set(user.email),
            phone: Set(user.phone),
            rules: Set(user.rules),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;

        Ok(())
    }

    // 创建用户信息
    async fn create_user(&self, user: UserQueryDto) -> anyhow::Result<()> {
        let model = entitiy::user_query::ActiveModel {
            user_id: Set(user.user_id),
            username: Set(user.username),
            email: Set(user.email),
            phone: Set(user.phone),
            rules: Set(user.rules),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    // 查询用户
    async fn find_user(&self, user_id: &str) -> anyhow::Result<UserQueryDto> {
        let user = entitiy::user_query::Entity::find()
            .filter(entitiy::user_query::Column::UserId.eq(user_id))
            .one(self.pool.as_ref())
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;

        Ok(user.into())
    }

    // 查询所有用户
    async fn find_all_user(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserQueryDto>> {
        let models = entitiy::user_query::Entity::find()
            .order_by_desc(entitiy::user_query::Column::CreatedAt)
            .offset((table_data_request.page - 1) * table_data_request.page_size)
            .limit(table_data_request.page_size)
            .all(self.pool.as_ref())
            .await?;

        let total = entitiy::user_query::Entity::find()
            .count(self.pool.as_ref())
            .await?;

        Ok(TableDataResponse {
            total,
            data: models.iter().map(|model| model.clone().into()).collect(),
        })
    }

    // 绑定角色
    async fn bind_roles(&self, user_id: String, roles: Vec<String>) -> anyhow::Result<()> {
        let model = entitiy::user_query::ActiveModel {
            user_id: Set(user_id),
            rules: Set(Some(serde_json::to_value(roles).unwrap())),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }
}
