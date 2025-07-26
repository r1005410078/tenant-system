use crate::domain::house::aggregates::house::HouseAggregate;

#[async_trait::async_trait]
pub trait HouseRepositoryAggregate: Send + Sync {
    // 创建小区
    async fn create(&self, aggregate: HouseAggregate) -> anyhow::Result<()>;
    // 更新小区
    async fn save(&self, aggregate: &HouseAggregate) -> anyhow::Result<()>;
    // 获取小区
    async fn find_by_id(&self, id: &str) -> anyhow::Result<HouseAggregate>;
    // 地址是否存在
    async fn exists_address(
        &self,
        community_id: &str,
        house_address: &str,
        self_id: Option<String>,
    ) -> anyhow::Result<bool>;
}
