use std::sync::Arc;

use sea_orm::{
    prelude::DateTimeUtc,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, DbConn, EntityTrait, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::community::value_objects::commuity::Community, infrastructure::entitiy::community_query,
};

pub struct CommunityQueryService {
    pool: Arc<DbConn>,
}

impl CommunityQueryService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        CommunityQueryService { pool }
    }

    // 创建小区
    pub async fn create(&self, event: Community) -> anyhow::Result<()> {
        let model = community_query::ActiveModel {
            id: Set(event.id.unwrap()),
            // 小区名称
            name: Set(event.name.unwrap()),
            // 小区编码
            adcode: Set(event.adcode),
            // 小区地址
            address: Set(event.address.unwrap()),
            // 区
            district: Set(event.district),
            // 城市
            city: Set(event.city.unwrap()),
            // 小区年限
            year_built: Set(event.year_built),
            // 小区类型
            typecode: Set(event.typecode.unwrap()),
            // 小区描述
            description: Set(event.description),
            // 小区图片
            images: Set(event.images.map(|v| serde_json::to_value(&v).unwrap())),
            // 位置
            lat: Set(event.lat.unwrap()),
            lng: Set(event.lng.unwrap()),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }
    // 更新小区
    pub async fn update(&self, event: Community) -> anyhow::Result<()> {
        let model = community_query::ActiveModel {
            id: Set(event.id.unwrap()),
            // 小区名称
            name: event.name.map_or(NotSet, Set),
            // 小区地址
            address: event.address.map_or(NotSet, Set),
            // 城市
            city: event.city.map_or(NotSet, Set),
            // 小区年限
            year_built: Set(event.year_built),
            // 小区类型
            typecode: event.typecode.map_or(NotSet, Set),
            // 小区描述
            description: Set(event.description),
            // 小区图片
            images: Set(event.images.map(|v| serde_json::to_value(&v).unwrap())),
            // 位置
            lat: event.lat.map_or(NotSet, Set),
            lng: event.lng.map_or(NotSet, Set),
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
        table_data_request: CommunityRequest,
    ) -> anyhow::Result<TableDataResponse<community_query::Model>> {
        let mut condition = Condition::all();

        if let Some(updated_at) = table_data_request.updated_at {
            condition = condition.add(community_query::Column::UpdatedAt.gt(updated_at));
        }

        let paginator = community_query::Entity::find()
            .filter(condition)
            .paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;
        let data = paginator
            .fetch_page(table_data_request.page.saturating_sub(1))
            .await?;

        Ok(TableDataResponse::new(data, total as u64))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommunityRequest {
    pub page: u64,
    pub page_size: u64,
    pub updated_at: Option<DateTimeUtc>,
}
