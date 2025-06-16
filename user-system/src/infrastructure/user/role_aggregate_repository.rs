use std::sync::Arc;

use crate::{
    application::repositories::role::RoleRepository,
    domain::roles::aggregates::role::RoleAggregate, infrastructure::entitiy::role_aggregate,
};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveValue::Set, DbConn};

pub struct MysqlUserRoleAggregateRepository {
    pool: Arc<DbConn>,
}

impl MysqlUserRoleAggregateRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MysqlUserRoleAggregateRepository { pool }
    }
}

#[async_trait::async_trait]
impl RoleRepository for MysqlUserRoleAggregateRepository {
    async fn create(&self, command: &RoleAggregate) -> anyhow::Result<RoleAggregate> {
        let model = role_aggregate::ActiveModel {
            id: Set(command.id.clone()),
            name: Set(command.name.clone()),
            description: Set(command.description.clone()),
            ..Default::default()
        };

        let result = model.insert(self.pool.as_ref()).await?;
        Ok(result.into())
    }

    async fn save(&self, command: &RoleAggregate) -> anyhow::Result<RoleAggregate> {
        let model = role_aggregate::ActiveModel {
            id: Set(command.id.clone()),
            name: Set(command.name.clone()),
            description: Set(command.description.clone()),
            ..Default::default()
        };

        let result = model.update(self.pool.as_ref()).await?;
        Ok(result.into())
    }

    async fn find_by_id(&self, id: &str) -> anyhow::Result<RoleAggregate> {
        let model = role_aggregate::Entity::find()
            .filter(
                Condition::all().add(
                    role_aggregate::Column::Id
                        .eq(id)
                        .add(role_aggregate::Column::DeletedAt.is_null()),
                ),
            )
            .one(self.pool.as_ref())
            .await?
            .ok_or(anyhow::anyhow!("role not found"));

        model.map(Into::into)
    }
}

impl From<role_aggregate::Model> for RoleAggregate {
    fn from(model: role_aggregate::Model) -> Self {
        RoleAggregate {
            id: model.id,
            name: model.name,
            description: model.description,
            permissions: Vec::new(),
            users: Vec::new(),
            deleted_at: model.deleted_at,
        }
    }
}
