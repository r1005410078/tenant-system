use std::sync::Arc;

use crate::{
    application::repositories::community_query_repository::CommunityQueryRepository,
    domain::community::events::{
        community_created::CommunityCreatedEvent, community_updated::CommunityUpdatedEvent,
    },
    infrastructure::{
        dtos::community_query_read_model_dto::CommunityQueryReadModelDto, entitiy::community_query,
    },
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn};
use sea_orm::{ActiveValue::NotSet, PaginatorTrait};
use sea_orm::{EntityTrait, QuerySelect};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

pub struct MySqlCommunityQueryRepository {
    pool: Arc<DbConn>,
}

impl MySqlCommunityQueryRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlCommunityQueryRepository { pool }
    }
}

#[async_trait::async_trait]
impl CommunityQueryRepository for MySqlCommunityQueryRepository {
    // 创建小区
    async fn create(&self, event: CommunityCreatedEvent) -> anyhow::Result<()> {
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
    async fn update(&self, event: CommunityUpdatedEvent) -> anyhow::Result<()> {
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
    async fn delete(&self, community_id: &str) -> anyhow::Result<()> {
        let model = community_query::ActiveModel {
            id: Set(community_id.to_string()),
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }

    // 查询小区列表
    async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<CommunityQueryReadModelDto>> {
        let data = community_query::Entity::find()
            .offset((table_data_request.page - 1) * table_data_request.page_size)
            .limit(table_data_request.page_size)
            .all(self.pool.as_ref())
            .await?;

        let total = community_query::Entity::find()
            .count(self.pool.as_ref())
            .await?;

        let records = data
            .into_iter()
            .map(|community| CommunityQueryReadModelDto::from(community))
            .collect();

        Ok(TableDataResponse::new(records, total as u64))
    }
}
