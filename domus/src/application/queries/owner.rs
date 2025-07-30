use std::sync::Arc;

use crate::{
    domain::owner::value_objects::owner::HouseOwner,
    infrastructure::entitiy::owner_query::{self},
};
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DbConn,
};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

pub struct OwnerQueryService {
    pool: Arc<DbConn>,
}

impl OwnerQueryService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        OwnerQueryService { pool }
    }

    // 创建业主
    pub async fn create(&self, event: HouseOwner) -> anyhow::Result<()> {
        let model = owner_query::ActiveModel {
            id: Set(event.id.unwrap().clone()),
            name: Set(event.name.unwrap().clone()),
            phone: Set(event.phone.unwrap().clone()),
            id_card: Set(event.id_card.clone()),
            id_card_images: Set(event
                .id_card_images
                .map(|v| serde_json::to_value(&v).unwrap())),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    // 更新业主
    pub async fn update(&self, event: HouseOwner) -> anyhow::Result<()> {
        let model = owner_query::ActiveModel {
            id: Set(event.id.unwrap().clone()),
            name: event.name.map_or(NotSet, Set),
            phone: event.phone.map_or(NotSet, Set),
            id_card: Set(event.id_card.clone()),
            id_card_images: Set(event
                .id_card_images
                .map(|v| serde_json::to_value(&v).unwrap())),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 删除业主
    pub async fn delete(&self, owner_id: &str) -> anyhow::Result<()> {
        let model = owner_query::ActiveModel {
            id: Set(owner_id.to_string()),
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }

    // 查询业主列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<owner_query::Model>> {
        let paginator =
            owner_query::Entity::find().paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;

        let data = paginator
            .fetch_page(table_data_request.page.saturating_sub(1))
            .await?;

        Ok(TableDataResponse::new(data, total as u64))
    }
}
