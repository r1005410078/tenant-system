use crate::domain::house::aggregates::house::HouseAggregate;

#[async_trait::async_trait]
pub trait HouseRepositoryAggregate {
    // 创建小区
    async fn create(&self, name: HouseAggregate) -> anyhow::Result<()>;
    // 更新小区
    async fn save(&self, name: &HouseAggregate) -> anyhow::Result<()>;
    // 获取小区
    async fn find_by_id(&self, id: &str) -> anyhow::Result<HouseAggregate>;
    // 小区是否存在
    async fn exists(&self, id: String) -> anyhow::Result<bool>;
}
