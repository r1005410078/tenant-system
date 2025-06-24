use std::sync::Arc;

use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::owner_query_repository::OwnerQueryRepository,
    domain::owner::events::{owner_created::OwnerCreatedEvent, owner_updated::OwnerUpdatedEvent},
    infrastructure::dtos::owner_query_read_model_dto::OwnerQueryReadModelDto,
};

pub struct OwnerQueryService {
    owner_query_repository: Arc<dyn OwnerQueryRepository>,
}

impl OwnerQueryService {
    pub fn new(owner_query_repository: Arc<dyn OwnerQueryRepository>) -> Self {
        OwnerQueryService {
            owner_query_repository,
        }
    }

    // 创建业主
    pub async fn create(&self, event: OwnerCreatedEvent) -> anyhow::Result<()> {
        self.owner_query_repository.create(event).await
    }

    // 更新业主
    pub async fn update(&self, event: OwnerUpdatedEvent) -> anyhow::Result<()> {
        self.owner_query_repository.update(event).await
    }

    // 删除业主
    pub async fn delete(&self, owner_id: &str) -> anyhow::Result<()> {
        self.owner_query_repository.delete(owner_id).await
    }

    // 查询业主列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<OwnerQueryReadModelDto>> {
        self.owner_query_repository
            .find_all(table_data_request)
            .await
    }
}
