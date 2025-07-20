use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder,
};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::user::events::{
        login::LoginEvent, user_binded_to_roles::UserBindedToRolesEvent,
        user_registered::UserRegisteredEvent, user_updated::UserUpdatedEvent,
    },
    infrastructure::{
        dtos::{user_login_history_dto::UserLoginHistoryDto, user_query_dto::UserQueryDto},
        entitiy::{self, sea_orm_active_enums::Status, user_login_history},
    },
};

pub struct UserQueryService {
    pool: Arc<DbConn>,
}

impl UserQueryService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }

    // 获取用户登录历史
    pub async fn get_user_login_history(
        &self,
        user_id: String,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserLoginHistoryDto>> {
        let paginator = user_login_history::Entity::find()
            .filter(user_login_history::Column::UserId.eq(user_id.clone()))
            .order_by_desc(user_login_history::Column::LoginAt)
            .paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;
        let models = paginator
            .fetch_page(table_data_request.page.saturating_sub(1))
            .await?;

        Ok(TableDataResponse::new(
            models.into_iter().map(Into::into).collect(),
            total,
        ))
    }

    // 保存用户登录历史
    pub async fn save_user_login_history(&self, login_event: &LoginEvent) -> anyhow::Result<()> {
        let user_login_history = match login_event {
            LoginEvent::Success(event) => {
                UserLoginHistoryDto::new(&event.user_id, &event.username, Status::Success)
            }
            LoginEvent::Fail(event) => {
                UserLoginHistoryDto::new(&event.user_id, &event.username, Status::Failure)
            }
        };

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

    // 保存用户信息
    pub async fn save_user(&self, user: &UserRegisteredEvent) -> anyhow::Result<()> {
        let model = entitiy::user_query::ActiveModel {
            user_id: Set(user.id.to_string()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            phone: Set(user.phone.clone()),
            roles: Set(Some(serde_json::to_value(user.roles.clone())?)),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    // 删除用户信息
    pub async fn delete_user(&self, user_id: &str) -> anyhow::Result<()> {
        entitiy::user_query::Entity::delete_many()
            .filter(entitiy::user_query::Column::UserId.eq(user_id))
            .exec(self.pool.as_ref())
            .await?;

        Ok(())
    }

    // 更新用户信息
    pub async fn update_user(&self, user: &UserUpdatedEvent) -> anyhow::Result<()> {
        let model = entitiy::user_query::ActiveModel {
            user_id: Set(user.id.to_string()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            phone: Set(user.phone.clone()),
            roles: Set(Some(serde_json::to_value(user.roles.clone())?)),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;

        Ok(())
    }

    // 查询用户
    pub async fn find_user(&self, user_id: &str) -> anyhow::Result<UserQueryDto> {
        let user = entitiy::user_query::Entity::find()
            .filter(entitiy::user_query::Column::UserId.eq(user_id))
            .one(self.pool.as_ref())
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;

        Ok(user.into())
    }

    // 查询所有用户
    pub async fn get_user_list(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserQueryDto>> {
        let paginator = entitiy::user_query::Entity::find()
            .order_by_desc(entitiy::user_query::Column::CreatedAt)
            .paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;
        let models = paginator
            .fetch_page(table_data_request.page.saturating_sub(1))
            .await?;

        Ok(TableDataResponse::new(
            models.into_iter().map(Into::into).collect(),
            total,
        ))
    }

    // 绑定角色
    pub async fn bind_roles(&self, event: &UserBindedToRolesEvent) -> anyhow::Result<()> {
        let model = entitiy::user_query::ActiveModel {
            user_id: Set(event.user_id.clone()),
            roles: Set(Some(serde_json::to_value(event.roles.clone()).unwrap())),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }
}
