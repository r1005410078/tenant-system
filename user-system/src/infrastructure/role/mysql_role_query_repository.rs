use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DbConn, EntityTrait, PaginatorTrait, QuerySelect,
};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::role_query_repository::RoleQueryRepository,
    domain::roles::events::permission_granted_to_role::Permission,
    infrastructure::{
        dtos::role_query_dto::RoleQueryReadModelDto, entitiy::role_detail_read_model,
    },
};

pub struct MysqlRoleQueryRepository {
    pool: Arc<DbConn>,
}

impl MysqlRoleQueryRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RoleQueryRepository for MysqlRoleQueryRepository {
    // 创建角色
    async fn create(&self, role: RoleQueryReadModelDto) -> anyhow::Result<()> {
        let model = role_detail_read_model::ActiveModel {
            id: Set(role.id),
            name: Set(role.name),
            description: Set(role.description),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;

        Ok(())
    }

    // 更新角色
    async fn update(&self, role: RoleQueryReadModelDto) -> anyhow::Result<()> {
        let model = role_detail_read_model::ActiveModel {
            id: Set(role.id),
            name: Set(role.name),
            description: Set(role.description),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 删除角色
    async fn delete(&self, role_id: &str) -> anyhow::Result<()> {
        let model = role_detail_read_model::ActiveModel {
            id: Set(role_id.to_string()),
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }
    // 查询角色
    async fn find(&self, role_id: &str) -> anyhow::Result<RoleQueryReadModelDto> {
        role_detail_read_model::Entity::find_by_id(role_id)
            .one(self.pool.as_ref())
            .await?
            .map(RoleQueryReadModelDto::from)
            .ok_or(anyhow::anyhow!("角色不存在"))
    }
    // 查询角色列表
    async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<RoleQueryReadModelDto>> {
        let data = role_detail_read_model::Entity::find()
            .offset((table_data_request.page - 1) * table_data_request.page_size)
            .limit(table_data_request.page_size)
            .all(self.pool.as_ref())
            .await?
            .into_iter()
            .map(RoleQueryReadModelDto::from)
            .collect::<Vec<RoleQueryReadModelDto>>();

        let total = role_detail_read_model::Entity::find()
            .count(self.pool.as_ref())
            .await?;

        Ok(TableDataResponse { total, data })
    }

    // 绑定角色权限
    async fn bind_permissions(
        &self,
        role_id: &str,
        permissions: Vec<Permission>,
    ) -> anyhow::Result<()> {
        role_detail_read_model::ActiveModel {
            id: Set(role_id.to_string()),
            permissions: Set(Some(serde_json::to_value(&permissions)?)),
            ..Default::default()
        }
        .update(self.pool.as_ref())
        .await?;

        Ok(())
    }
}
