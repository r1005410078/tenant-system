use crate::domain::owner::aggregates::owner::OwnerAggregate;

#[async_trait::async_trait]
pub trait OwnerRepositoryAggregate: Send + Sync {
    // 创建owner
    async fn create(&self, name: OwnerAggregate) -> anyhow::Result<()>;
    // 更新owner
    async fn save(&self, name: &OwnerAggregate) -> anyhow::Result<()>;
    // 获取owner
    async fn find_by_id(&self, id: &str) -> anyhow::Result<OwnerAggregate>;
    // 根据手机号获取owner
    async fn find_by_phone(&self, phone: &str) -> anyhow::Result<Option<OwnerAggregate>>;
    // 手机号是否存在
    async fn exists_phone(&self, phone: &str, self_id: Option<String>) -> anyhow::Result<bool>;
    // 身份证号是否存在
    async fn exists_id_card(&self, id_card: &str, self_id: Option<String>) -> anyhow::Result<bool>;
}
