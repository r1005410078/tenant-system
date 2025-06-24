use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::house::events::{house_created::HouseCreatedEvent, house_updated::HouseUpdatedEvent},
    infrastructure::dtos::house_query_read_model_dto::HouseQueryReadModelDto,
};

#[async_trait::async_trait]
pub trait HouseQueryRepository: Send + Sync {
    // 创建房源
    async fn create(&self, event: HouseCreatedEvent) -> anyhow::Result<()>;
    // 更新房源
    async fn update(&self, event: HouseUpdatedEvent) -> anyhow::Result<()>;
    // 删除房源
    async fn delete(&self, house_id: &str) -> anyhow::Result<()>;
    // 查询房源列表
    async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<HouseQueryReadModelDto>>;

    // 发布房源
    async fn publish(&self, house_id: &str) -> anyhow::Result<()>;
    // 取消发布房源
    async fn unpublish(&self, house_id: &str) -> anyhow::Result<()>;
}
