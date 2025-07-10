use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbConn, EntityTrait,
    PaginatorTrait, QueryFilter,
};

use crate::{
    application::repositories::house_repository_aggregate::HouseRepositoryAggregate,
    domain::house::aggregates::house::HouseAggregate, infrastructure::entitiy,
};

pub struct MysqlHouseRepositoryAggregate {
    pub pool: Arc<DbConn>,
}

impl MysqlHouseRepositoryAggregate {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl HouseRepositoryAggregate for MysqlHouseRepositoryAggregate {
    // 创建小区
    async fn create(&self, aggregate: HouseAggregate) -> anyhow::Result<()> {
        let model = entitiy::house_aggregate::ActiveModel {
            house_id: Set(aggregate.house_id),
            community_id: Set(aggregate.community_id),
            door_number: Set(aggregate.door_number),
            publish_at: Set(aggregate.publish_at),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    // 更新小区
    async fn save(&self, aggregate: &HouseAggregate) -> anyhow::Result<()> {
        let model = entitiy::house_aggregate::ActiveModel {
            house_id: Set(aggregate.house_id.clone()),
            community_id: Set(aggregate.community_id.clone()),
            door_number: Set(aggregate.door_number.clone()),
            publish_at: Set(aggregate.publish_at.clone()),
            unpublish_at: Set(aggregate.unpublish_at.clone()),
            deleted_at: Set(aggregate.deleted_at.clone()),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }
    // 获取小区
    async fn find_by_id(&self, id: &str) -> anyhow::Result<HouseAggregate> {
        entitiy::house_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::house_aggregate::Column::HouseId.eq(id))
                    .add(entitiy::house_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await?
            .map(|model| HouseAggregate {
                house_id: model.house_id,
                community_id: model.community_id,
                door_number: model.door_number,
                publish_at: model.publish_at,
                unpublish_at: model.unpublish_at,
                deleted_at: model.deleted_at,
            })
            .ok_or_else(|| anyhow::anyhow!("房源不存在"))
    }

    // 地址是否存在
    async fn exists_address(
        &self,
        community_id: &str,
        door_number: Option<String>,
        self_id: Option<String>,
    ) -> anyhow::Result<bool> {
        let mut condition = Condition::all()
            .add(entitiy::house_aggregate::Column::CommunityId.eq(community_id))
            .add(entitiy::house_aggregate::Column::DoorNumber.eq(door_number))
            .add(entitiy::house_aggregate::Column::DeletedAt.is_null());

        if let Some(id) = self_id {
            condition = condition.add(entitiy::house_aggregate::Column::HouseId.ne(id));
        }

        let count = entitiy::house_aggregate::Entity::find()
            .filter(condition)
            .count(self.pool.as_ref())
            .await?;

        Ok(count > 0)
    }
}
