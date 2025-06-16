use crate::domain::roles::aggregates::role::RoleAggregate;

#[async_trait::async_trait]
pub trait RoleRepository {
    async fn create(&self, command: &RoleAggregate) -> anyhow::Result<RoleAggregate>;
    async fn save(&self, command: &RoleAggregate) -> anyhow::Result<RoleAggregate>;
    async fn find_by_id(&self, id: &str) -> anyhow::Result<RoleAggregate>;
}
