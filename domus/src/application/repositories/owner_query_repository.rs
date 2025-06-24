use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::owner::events::{owner_created::OwnerCreatedEvent, owner_updated::OwnerUpdatedEvent},
    infrastructure::dtos::owner_query_read_model_dto::OwnerQueryReadModelDto,
};

#[async_trait::async_trait]
pub trait OwnerQueryRepository: Send + Sync {
    // 创建业主
    async fn create(&self, event: OwnerCreatedEvent) -> anyhow::Result<()>;
    // 更新业主
    async fn update(&self, event: OwnerUpdatedEvent) -> anyhow::Result<()>;
    // 删除业主
    async fn delete(&self, owner_id: &str) -> anyhow::Result<()>;
    // 查询业主列表
    async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<OwnerQueryReadModelDto>>;
}
