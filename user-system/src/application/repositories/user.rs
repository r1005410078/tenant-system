use crate::domain::user::aggregates::user::UserAggregate;

#[async_trait::async_trait]
pub trait UserAggregateRepository: Send + Sync {
    // 保存用户
    async fn save(&self, user: &UserAggregate) -> anyhow::Result<()>;
    async fn create(&self, user: &UserAggregate) -> anyhow::Result<()>;
    // 根据用户ID查找用户
    async fn find_by_id(&self, user_id: &str) -> Option<UserAggregate>;
    async fn find_by_username(&self, username: &str) -> Option<UserAggregate>;
    // 用户是否存在
    async fn exists(&self, username: &str) -> anyhow::Result<bool>;
}
