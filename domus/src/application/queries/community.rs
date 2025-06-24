use std::sync::Arc;

use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::community_query_repository::CommunityQueryRepository,
    domain::community::events::{
        community_created::CommunityCreatedEvent, community_updated::CommunityUpdatedEvent,
    },
    infrastructure::dtos::community_query_read_model_dto::CommunityQueryReadModelDto,
};

pub struct CommunityQueryService {
    community_query_repository: Arc<dyn CommunityQueryRepository>,
}

impl CommunityQueryService {
    pub fn new(community_query_repository: Arc<dyn CommunityQueryRepository>) -> Self {
        CommunityQueryService {
            community_query_repository,
        }
    }

    // 创建小区
    pub async fn create(&self, event: CommunityCreatedEvent) -> anyhow::Result<()> {
        self.community_query_repository.create(event).await
    }
    // 更新小区
    pub async fn update(&self, event: CommunityUpdatedEvent) -> anyhow::Result<()> {
        self.community_query_repository.update(event).await
    }

    // 删除小区
    pub async fn delete(&self, community_id: &str) -> anyhow::Result<()> {
        self.community_query_repository.delete(community_id).await
    }

    // 查询小区列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<CommunityQueryReadModelDto>> {
        self.community_query_repository
            .find_all(table_data_request)
            .await
    }
}
