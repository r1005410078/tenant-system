use std::sync::Arc;

use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::house_query_repository::HouseQueryRepository,
    domain::house::events::{house_created::HouseCreatedEvent, house_updated::HouseUpdatedEvent},
    infrastructure::dtos::house_query_read_model_dto::HouseQueryReadModelDto,
};

pub struct HouseQueryService {
    house_query_repository: Arc<dyn HouseQueryRepository>,
}

impl HouseQueryService {
    pub fn new(house_query_repository: Arc<dyn HouseQueryRepository>) -> Self {
        HouseQueryService {
            house_query_repository,
        }
    }

    // 创建房源
    pub async fn create(&self, event: HouseCreatedEvent) -> anyhow::Result<()> {
        self.house_query_repository.create(event).await
    }

    // 更新房源
    pub async fn update(&self, event: HouseUpdatedEvent) -> anyhow::Result<()> {
        self.house_query_repository.update(event).await
    }

    // 删除房源
    pub async fn delete(&self, house_id: &str) -> anyhow::Result<()> {
        self.house_query_repository.delete(house_id).await
    }

    // 查询房源列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<HouseQueryReadModelDto>> {
        self.house_query_repository
            .find_all(table_data_request)
            .await
    }

    // 发布房源
    pub async fn publish(&self, house_id: &str) -> anyhow::Result<()> {
        self.house_query_repository.publish(house_id).await
    }

    // 取消发布房源
    pub async fn unpublish(&self, house_id: &str) -> anyhow::Result<()> {
        self.house_query_repository.unpublish(house_id).await
    }
}
