use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::infrastructure::dtos::{
    user_login_history_dto::UserLoginHistoryDto, user_query_dto::UserQueryDto,
};

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

    // 保存用户信息
    async fn create_user(&self, user: UserQueryDto) -> anyhow::Result<()>;

    // 删除用户信息
    async fn delete_user(&self, user_id: &str) -> anyhow::Result<()>;

    // 更新用户信息
    async fn update_user(&self, user: UserQueryDto) -> anyhow::Result<()>;

    // 查询用户
    async fn find_user(&self, user_id: &str) -> anyhow::Result<UserQueryDto>;

    // 查询所有用户
    async fn find_all_user(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserQueryDto>>;

    // 绑定角色
    async fn bind_roles(&self, user_id: String, roles: Vec<String>) -> anyhow::Result<()>;
}
