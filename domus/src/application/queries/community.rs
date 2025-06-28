use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DbConn, EntityTrait, PaginatorTrait,
};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::community::events::{
        community_created::CommunityCreatedEvent, community_updated::CommunityUpdatedEvent,
    },
    infrastructure::{
        dtos::community_query_read_model_dto::CommunityQueryReadModelDto, entitiy::community_query,
    },
};

pub struct CommunityQueryService {
    pool: Arc<DbConn>,
}

impl CommunityQueryService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        CommunityQueryService { pool }
    }

    // 创建小区
    pub async fn create(&self, event: CommunityCreatedEvent) -> anyhow::Result<()> {
        let model = community_query::ActiveModel {
            id: Set(event.community_id),
            // 小区名称
            name: Set(event.name),
            // 小区地址
            address: Set(event.address),
            // 城市
            city: Set(event.city),
            // 小区年限
            year_built: Set(event.year_built),
            // 小区类型
            community_type: Set(event.community_type),
            // 小区描述
            description: Set(event.description),
            // 小区图片
            image: Set(event.image),
            // 位置
            location: Set(event.location),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }
    // 更新小区
    pub async fn update(&self, event: CommunityUpdatedEvent) -> anyhow::Result<()> {
        let model = community_query::ActiveModel {
            id: Set(event.community_id),
            // 小区名称
            name: event.name.map_or(NotSet, Set),
            // 小区地址
            address: event.address.map_or(NotSet, Set),
            // 城市
            city: event.city.map_or(NotSet, Set),
            // 小区年限
            year_built: event.year_built.map_or(NotSet, Set),
            // 小区类型
            community_type: event.community_type.map_or(NotSet, Set),
            // 小区描述
            description: Set(event.description),
            // 小区图片
            image: Set(event.image),
            // 位置
            location: Set(event.location),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 删除小区
    pub async fn delete(&self, community_id: &str) -> anyhow::Result<()> {
        let model = community_query::ActiveModel {
            id: Set(community_id.to_string()),
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }

    // 查询小区列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<CommunityQueryReadModelDto>> {
        let paginator = community_query::Entity::find()
            .paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;
        let data = paginator
            .fetch_page(table_data_request.page.saturating_sub(1))
            .await?;

        let records = data
            .into_iter()
            .map(CommunityQueryReadModelDto::from)
            .collect();

        Ok(TableDataResponse::new(records, total as u64))
    }
}
