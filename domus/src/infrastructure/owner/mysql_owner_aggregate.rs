use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbConn, EntityTrait,
    PaginatorTrait, QueryFilter,
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
            phone: Set(aggregate.phone),
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
            phone: Set(aggregate.phone.clone()),
            id_card: Set(aggregate.id_card.clone()),
            deleted_at: Set(aggregate.deleted_at.clone()),
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
            .ok_or_else(|| anyhow::anyhow!("业主不存在"))?;

        Ok(OwnerAggregate {
            owner_id: model.owner_id,
            name: model.name,
            phone: model.phone,
            id_card: model.id_card,
            deleted_at: model.deleted_at,
        })
    }

    // 是否存在
    async fn exists_phone(&self, phone: &str, self_id: Option<String>) -> anyhow::Result<bool> {
        let mut condition = Condition::all()
            .add(entitiy::owner::Column::Phone.eq(phone))
            .add(entitiy::owner::Column::DeletedAt.is_null());

        // 如果有self_id，则排除当前记录
        if let Some(id) = self_id {
            condition = condition.add(entitiy::owner::Column::OwnerId.ne(id));
        }

        let m = entitiy::owner::Entity::find()
            .filter(condition.clone())
            .all(self.pool.as_ref())
            .await?;

        println!("m: {:?}", m);

        let count = entitiy::owner::Entity::find()
            .filter(condition)
            .count(self.pool.as_ref())
            .await?;

        Ok(count > 0)
    }

    // 身份证号是否存在
    async fn exists_id_card(&self, id_card: &str, self_id: Option<String>) -> anyhow::Result<bool> {
        let mut condition = Condition::all()
            .add(entitiy::owner::Column::IdCard.eq(id_card))
            .add(entitiy::owner::Column::DeletedAt.is_null());

        if let Some(id) = self_id {
            condition = condition.add(entitiy::owner::Column::OwnerId.ne(id));
        }

        let count = entitiy::owner::Entity::find()
            .filter(condition)
            .count(self.pool.as_ref())
            .await?;
        Ok(count > 0)
    }
}
