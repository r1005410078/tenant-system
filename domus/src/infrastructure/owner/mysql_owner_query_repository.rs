use std::sync::Arc;

use crate::{
    application::repositories::owner_query_repository::OwnerQueryRepository,
    domain::owner::events::{owner_created::OwnerCreatedEvent, owner_updated::OwnerUpdatedEvent},
    infrastructure::{
        dtos::owner_query_read_model_dto::OwnerQueryReadModelDto,
        entitiy::{owner, owner_query},
    },
};
use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DbConn,
};
use sea_orm::{EntityTrait, QuerySelect};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};
pub struct MySqlOwnerQueryRepository {
    pool: Arc<DbConn>,
}

impl MySqlOwnerQueryRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlOwnerQueryRepository { pool }
    }
}

#[async_trait::async_trait]
impl OwnerQueryRepository for MySqlOwnerQueryRepository {
    // 创建业主
    async fn create(&self, event: OwnerCreatedEvent) -> anyhow::Result<()> {
        let model = owner_query::ActiveModel {
            id: Set(event.id.clone()),
            name: Set(event.name.clone()),
            phone: Set(event.phone.clone()),
            id_card: Set(event.id_card.clone()),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }
    // 更新业主
    async fn update(&self, event: OwnerUpdatedEvent) -> anyhow::Result<()> {
        let model = owner_query::ActiveModel {
            id: Set(event.id.clone()),
            name: event.name.map_or(NotSet, Set),
            phone: event.phone.map_or(NotSet, Set),
            id_card: Set(event.id_card.clone()),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }
    // 删除业主
    async fn delete(&self, owner_id: &str) -> anyhow::Result<()> {
        let model = owner_query::ActiveModel {
            id: Set(owner_id.to_string()),
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }
    // 查询业主列表
    async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<OwnerQueryReadModelDto>> {
        let paginator =
            owner_query::Entity::find().paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;
        let data = paginator
            .fetch_page(table_data_request.page - 1)
            .await?
            .into_iter()
            .map(OwnerQueryReadModelDto::from)
            .collect::<Vec<OwnerQueryReadModelDto>>();

        Ok(TableDataResponse::new(data, total as u64))
    }
}
