use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::infrastructure::dtos::user_login_history_dto::UserLoginHistoryDto;

#[async_trait::async_trait]
pub trait UserQueryRepository: Send + Sync {
    // 查询所以用户登录历史
    async fn get_user_login_history(
        &self,
        user_id: &str,
        table_data_request: &TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserLoginHistoryDto>>;

    // 保持用户登录历史
    async fn save_user_login_history(
        &self,
        user_login_history: UserLoginHistoryDto,
    ) -> anyhow::Result<()>;
}
