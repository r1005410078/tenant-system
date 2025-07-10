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
    async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<CommunityAggregate>> {
        let data = entitiy::community_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::community_aggregate::Column::Id.eq(id))
                    .add(entitiy::community_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await?
            .map(|m| CommunityAggregate {
                community_id: m.id,
                name: m.name,
                address: m.address,
                deleted_at: m.deleted_at,
            });

        Ok(data)
    }
}
