use crate::domain::owner::aggregates::owner::OwnerAggregate;

#[async_trait::async_trait]
pub trait OwnerRepositoryAggregate: Send + Sync {
    // 创建owner
    async fn create(&self, name: OwnerAggregate) -> anyhow::Result<()>;
    // 更新owner
    async fn save(&self, name: &OwnerAggregate) -> anyhow::Result<()>;
    // 获取owner
    async fn find_by_id(&self, id: &str) -> anyhow::Result<OwnerAggregate>;
}
