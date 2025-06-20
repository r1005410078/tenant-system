use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbConn, EntityTrait, QueryFilter,
};

use crate::{
    application::repositories::owner_repository_aggregate::OwnerRepositoryAggregate,
    domain::owner::aggregates::owner::OwnerAggregate, infrastructure::entitiy,
};

pub struct MySqlOwnerAggregateRepository {
    pool: Arc<DbConn>,
}

impl MySqlOwnerAggregateRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlOwnerAggregateRepository { pool }
    }
}

#[async_trait::async_trait]
impl OwnerRepositoryAggregate for MySqlOwnerAggregateRepository {
    // 创建owner
    async fn create(&self, aggregate: OwnerAggregate) -> anyhow::Result<()> {
        let model = entitiy::owner::ActiveModel {
            owner_id: Set(aggregate.owner_id),
            name: Set(aggregate.name),
            id_card: Set(aggregate.id_card),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }
    // 更新owner
    async fn save(&self, aggregate: &OwnerAggregate) -> anyhow::Result<()> {
        let model = entitiy::owner::ActiveModel {
            owner_id: Set(aggregate.owner_id.clone()),
            name: Set(aggregate.name.clone()),
            id_card: Set(aggregate.id_card.clone()),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 获取owner
    async fn find_by_id(&self, id: &str) -> anyhow::Result<OwnerAggregate> {
        let model = entitiy::owner::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::owner::Column::OwnerId.eq(id))
                    .add(entitiy::owner::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Owner not found"))?;

        Ok(OwnerAggregate {
            owner_id: model.owner_id,
            name: model.name,
            id_card: model.id_card,
            deleted_at: model.deleted_at,
        })
    }
}
