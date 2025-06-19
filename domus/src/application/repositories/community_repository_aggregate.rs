use crate::domain::community::aggregates::community::CommunityAggregate;

#[async_trait::async_trait]
pub trait CommunityRepositoryAggregate: Send + Sync {
    // 创建小区
    async fn create(&self, name: CommunityAggregate) -> anyhow::Result<()>;
    // 更新小区
    async fn save(&self, name: &CommunityAggregate) -> anyhow::Result<()>;
    // 获取小区
    async fn find_by_id(&self, id: &str) -> anyhow::Result<CommunityAggregate>;
    // 小区是否存在
    async fn exists(&self, id: String) -> anyhow::Result<bool>;
}
