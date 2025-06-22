use std::sync::Arc;

use sea_orm::sea_query::Table;
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::user_query_repository::UserQueryRepository,
    domain::user::events::{
        login::LoginEvent, user_binded_to_roles::UserBindedToRolesEvent,
        user_registered::UserRegisteredEvent, user_updated::UserUpdatedEvent,
    },
    infrastructure::{
        dtos::{user_login_history_dto::UserLoginHistoryDto, user_query_dto::UserQueryDto},
        entitiy::sea_orm_active_enums::Status,
    },
};

pub struct UserQueryService {
    user_query_repository: Arc<dyn UserQueryRepository>,
}

impl UserQueryService {
    pub fn new(user_query_repository: Arc<dyn UserQueryRepository>) -> Self {
        Self {
            user_query_repository,
        }
    }

    // 获取用户登录历史
    pub async fn get_user_login_history(
        &self,
        user_id: String,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserLoginHistoryDto>> {
        self.user_query_repository
            .get_user_login_history(&user_id, &table_data_request)
            .await
    }

    // 保存用户登录历史
    pub async fn save_user_login_history(&self, login_event: &LoginEvent) -> anyhow::Result<()> {
        let user_login_history_dto = match login_event {
            LoginEvent::Success(event) => {
                UserLoginHistoryDto::new(&event.user_id, &event.username, Status::Success)
            }
            LoginEvent::Fail(event) => {
                UserLoginHistoryDto::new(&event.user_id, &event.username, Status::Failure)
            }
        };

        self.user_query_repository
            .save_user_login_history(user_login_history_dto)
            .await
    }

    // 保存用户信息
    pub async fn save_user(&self, user: &UserRegisteredEvent) -> anyhow::Result<()> {
        self.user_query_repository
            .create_user(user.to_user_query_dto())
            .await
    }

    // 删除用户信息
    pub async fn delete_user(&self, user_id: &str) -> anyhow::Result<()> {
        self.user_query_repository.delete_user(user_id).await
    }

    // 更新用户信息
    pub async fn update_user(&self, user: &UserUpdatedEvent) -> anyhow::Result<()> {
        self.user_query_repository
            .update_user(user.to_user_query_dto())
            .await
    }

    // 查询用户
    pub async fn find_user(&self, user_id: &str) -> anyhow::Result<UserQueryDto> {
        self.user_query_repository.find_user(user_id).await
    }

    // 查询所有用户
    pub async fn get_user_list(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserQueryDto>> {
        self.user_query_repository
            .find_all_user(table_data_request)
            .await
    }

    // 绑定角色
    pub async fn bind_roles(&self, event: &UserBindedToRolesEvent) -> anyhow::Result<()> {
        self.user_query_repository
            .bind_roles(event.user_id.clone(), event.roles.clone())
            .await
    }
}
