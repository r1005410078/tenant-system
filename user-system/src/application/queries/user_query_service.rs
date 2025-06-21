use std::sync::Arc;

use sea_orm::sea_query::Table;
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::user_query_repository::UserQueryRepository,
    domain::user::events::login::LoginEvent,
    infrastructure::{
        dtos::user_login_history_dto::UserLoginHistoryDto, entitiy::sea_orm_active_enums::Status,
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
}
