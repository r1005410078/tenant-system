use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, EntityTrait, PaginatorTrait};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::roles::events::{
        permission_granted_to_role::PermissionGrantedToRoleEvent, role_created::RoleCreatedEvent,
        role_deleted::RoleDeletedEvent, role_updated::RoleUpdatedEvent,
    },
    infrastructure::{
        dtos::role_query_dto::RoleQueryReadModelDto, entitiy::role_detail_read_model,
    },
};
use std::sync::Arc;

pub struct RoleQueryService {
    pool: Arc<DbConn>,
}

impl RoleQueryService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }

    // 创建角色
    pub async fn create(&self, role: RoleCreatedEvent) -> anyhow::Result<()> {
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
    pub async fn update(&self, role: RoleUpdatedEvent) -> anyhow::Result<()> {
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
    pub async fn delete(&self, event: RoleDeletedEvent) -> anyhow::Result<()> {
        let model = role_detail_read_model::ActiveModel {
            id: Set(event.id),
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }
    // 查询角色
    pub async fn find(&self, role_id: &str) -> anyhow::Result<RoleQueryReadModelDto> {
        role_detail_read_model::Entity::find_by_id(role_id)
            .one(self.pool.as_ref())
            .await?
            .map(RoleQueryReadModelDto::from)
            .ok_or(anyhow::anyhow!("角色不存在"))
    }

    // 查询角色列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<RoleQueryReadModelDto>> {
        let paginator = role_detail_read_model::Entity::find()
            .paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;
        let data = paginator
            .fetch_page(table_data_request.page.saturating_sub(1))
            .await?
            .into_iter()
            .map(RoleQueryReadModelDto::from)
            .collect::<Vec<_>>();

        Ok(TableDataResponse::new(data, total))
    }

    // 绑定权限
    pub async fn bind_permissions(
        &self,
        event: &PermissionGrantedToRoleEvent,
    ) -> anyhow::Result<()> {
        role_detail_read_model::ActiveModel {
            id: Set(event.role_id.clone()),
            permissions: Set(Some(serde_json::to_value(&event.permissions)?)),
            ..Default::default()
        }
        .update(self.pool.as_ref())
        .await?;

        Ok(())
    }
}
