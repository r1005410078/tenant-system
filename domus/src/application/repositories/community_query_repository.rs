use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::community::events::{
        community_created::CommunityCreatedEvent, community_updated::CommunityUpdatedEvent,
    },
    infrastructure::dtos::community_query_read_model_dto::CommunityQueryReadModelDto,
};

#[async_trait::async_trait]
pub trait CommunityQueryRepository: Send + Sync {
    // 创建小区
    async fn create(&self, event: CommunityCreatedEvent) -> anyhow::Result<()>;
    // 更新小区
    async fn update(&self, event: CommunityUpdatedEvent) -> anyhow::Result<()>;
    // 删除小区
    async fn delete(&self, community_id: &str) -> anyhow::Result<()>;
    // 查询小区列表
    async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<CommunityQueryReadModelDto>>;
}
