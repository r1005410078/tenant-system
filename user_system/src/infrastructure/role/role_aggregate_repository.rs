use std::sync::Arc;

use crate::domain::roles::events::permission_granted_to_role::Permission;
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
use user_system::shared::entitiy::casbin_rules;

pub struct MySqlRoleAggregateRepository {
    pool: Arc<DbConn>,
}

impl MySqlRoleAggregateRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlRoleAggregateRepository { pool }
    }

    async fn get_permissions_by_role_id(&self, role_id: &str) -> anyhow::Result<Vec<Permission>> {
        let models = casbin_rules::Entity::find()
            .filter(casbin_rules::Column::V0.eq(role_id))
            .all(self.pool.as_ref())
            .await?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Permission>>();

        Ok(models)
    }
}

#[async_trait::async_trait]
impl RoleRepository for MySqlRoleAggregateRepository {
    async fn create(&self, command: &RoleAggregate) -> anyhow::Result<RoleAggregate> {
        let model = role_aggregate::ActiveModel {
            id: Set(command.id.clone()),
            name: Set(command.name.clone()),
            description: Set(command.description.clone()),
            ..Default::default()
        };

        let mut result: RoleAggregate = model.insert(self.pool.as_ref()).await?.into();
        // 创建权限
        for p in command.permissions.iter() {
            // 创建权限关联
            let model = casbin_rules::ActiveModel {
                ptype: Set("p".to_string()),
                v0: Set(Some(command.id.clone())),
                v1: Set(Some(p.source.clone())),
                v2: Set(Some(p.action.clone())),
                ..Default::default()
            };
            model.insert(self.pool.as_ref()).await?;
        }

        result.permissions = self.get_permissions_by_role_id(&command.id).await?;
        Ok(result)
    }

    async fn save(&self, command: &RoleAggregate) -> anyhow::Result<RoleAggregate> {
        let model = role_aggregate::ActiveModel {
            id: Set(command.id.clone()),
            name: Set(command.name.clone()),
            description: Set(command.description.clone()),
            deleted_at: Set(command.deleted_at.clone()),
            ..Default::default()
        };

        let mut result: RoleAggregate = model.update(self.pool.as_ref()).await?.into();

        // 删除旧的权限
        casbin_rules::Entity::delete_many()
            .filter(casbin_rules::Column::V0.eq(command.id.clone()))
            .exec(self.pool.as_ref())
            .await?;

        // 更新权限
        for p in command.permissions.iter() {
            // 创建权限关联
            let model = casbin_rules::ActiveModel {
                ptype: Set("p".to_string()),
                v0: Set(Some(command.id.to_string())),
                v1: Set(Some(p.source.clone())),
                v2: Set(Some(p.action.clone())),
                ..Default::default()
            };
            model.insert(self.pool.as_ref()).await?;
        }

        result.permissions = self.get_permissions_by_role_id(&command.id).await?;
        Ok(result)
    }

    async fn find_by_id(&self, id: &str) -> anyhow::Result<RoleAggregate> {
        let mut model: RoleAggregate = role_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(role_aggregate::Column::Id.eq(id))
                    .add(role_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await?
            .ok_or(anyhow::anyhow!(format!("找不到角色: {}", id)))?
            .into();

        model.permissions = self.get_permissions_by_role_id(&model.id).await?;
        Ok(model)
    }
}

impl From<role_aggregate::Model> for RoleAggregate {
    fn from(model: role_aggregate::Model) -> Self {
        RoleAggregate {
            id: model.id,
            name: model.name,
            description: model.description,
            permissions: Vec::new(),
            deleted_at: model.deleted_at,
        }
    }
}

impl From<casbin_rules::Model> for Permission {
    fn from(model: casbin_rules::Model) -> Self {
        Permission {
            source: model.v1.unwrap(),
            action: model.v2.unwrap(),
        }
    }
}
