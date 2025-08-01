use crate::domain::community::aggregates::community::CommunityAggregate;

#[async_trait::async_trait]
pub trait CommunityRepositoryAggregate: Send + Sync {
    // 创建小区
    async fn create(&self, aggregate: CommunityAggregate) -> anyhow::Result<()>;
    // 更新小区
    async fn save(&self, aggregate: &CommunityAggregate) -> anyhow::Result<()>;
    // 获取小区
    async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<CommunityAggregate>>;
}
