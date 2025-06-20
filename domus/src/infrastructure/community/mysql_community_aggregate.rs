use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbConn, EntityTrait, QueryFilter,
};
use std::sync::Arc;

use crate::{
    application::repositories::community_repository_aggregate::CommunityRepositoryAggregate,
    domain::community::aggregates::community::CommunityAggregate, infrastructure::entitiy,
};

pub struct MySqlCommunityAggregateRepository {
    pool: Arc<DbConn>,
}

impl MySqlCommunityAggregateRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlCommunityAggregateRepository { pool }
    }
}

#[async_trait::async_trait]
impl CommunityRepositoryAggregate for MySqlCommunityAggregateRepository {
    // 创建小区
    async fn create(&self, community: CommunityAggregate) -> anyhow::Result<()> {
        let model = entitiy::community_aggregate::ActiveModel {
            id: Set(community.community_id),
            name: Set(community.name),
            address: Set(community.address),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    // 更新小区
    async fn save(&self, community: &CommunityAggregate) -> anyhow::Result<()> {
        let model = entitiy::community_aggregate::ActiveModel {
            id: Set(community.community_id.clone()),
            name: Set(community.name.clone()),
            address: Set(community.address.clone()),
            deleted_at: Set(community.deleted_at.clone()),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 获取小区
    async fn find_by_id(&self, id: &str) -> anyhow::Result<CommunityAggregate> {
        let model = entitiy::community_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::community_aggregate::Column::Id.eq(id))
                    .add(entitiy::community_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Community not found"))?;

        Ok(CommunityAggregate {
            community_id: model.id,
            name: model.name,
            address: model.address,
            deleted_at: model.deleted_at,
        })
    }

    // 判断小区是否重复
    async fn exists(&self, address: &str) -> anyhow::Result<bool> {
        let count = entitiy::community_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::community_aggregate::Column::Address.eq(address))
                    .add(entitiy::community_aggregate::Column::DeletedAt.is_null()),
            )
            .count(self.pool.as_ref())
            .await?;

        Ok(count > 0)
    }
}
