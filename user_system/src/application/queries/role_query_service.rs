use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, EntityTrait, PaginatorTrait};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::roles::events::{
        permission_granted_to_role::{Permission, PermissionGrantedToRoleEvent},
        role_created::RoleCreatedEvent,
        role_deleted::RoleDeletedEvent,
        role_updated::RoleUpdatedEvent,
    },
    infrastructure::{
        dtos::role_query_dto::RoleQueryReadModelDto,
        entitiy::{permissions_detail, role_detail_read_model},
    },
};
use std::{collections::HashMap, sync::Arc};

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

    // 获取
    pub async fn get_permissions_details_map(
        &self,
    ) -> anyhow::Result<HashMap<Permission, permissions_detail::Model>> {
        let mut permissions_details_map = HashMap::new();
        let permissions_details = permissions_detail::Entity::find()
            .all(self.pool.as_ref())
            .await?;

        for detail in permissions_details {
            permissions_details_map.insert(
                Permission {
                    action: detail.action.to_string(),
                    source: detail.source.to_string(),
                },
                detail,
            );
        }

        Ok(permissions_details_map)
    }

    // 查询角色
    pub async fn find(&self, role_id: &str) -> anyhow::Result<RoleQueryReadModelDto> {
        let role = role_detail_read_model::Entity::find_by_id(role_id)
            .one(self.pool.as_ref())
            .await?
            .ok_or(anyhow::anyhow!("角色不存在"))?;

        let permissions_details_map = self.get_permissions_details_map().await?;

        let permissions = role.permissions.map(|permissions| {
            let permissions: Vec<Permission> =
                serde_json::from_value(permissions).unwrap_or_default();
            permissions
                .iter()
                .map(|p| permissions_details_map.get(p).cloned())
                .flatten()
                .collect::<Vec<permissions_detail::Model>>()
        });

        let role_query = RoleQueryReadModelDto {
            id: role.id,
            name: role.name,
            description: role.description,
            permissions,
            created_at: role.created_at,
            updated_at: role.updated_at,
        };

        Ok(role_query)
    }

    // 查询角色列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<RoleQueryReadModelDto>> {
        let paginator = role_detail_read_model::Entity::find()
            .paginate(self.pool.as_ref(), table_data_request.page_size);

        let total = paginator.num_items().await?;
        let role_list = paginator
            .fetch_page(table_data_request.page.saturating_sub(1))
            .await?;

        let permissions_details_map = self.get_permissions_details_map().await?;

        let mut data = vec![];

        for role in role_list {
            let permissions = role.permissions.map(|permissions| {
                let permissions: Vec<Permission> =
                    serde_json::from_value(permissions).unwrap_or_default();
                permissions
                    .iter()
                    .map(|p| permissions_details_map.get(p).cloned())
                    .flatten()
                    .collect::<Vec<permissions_detail::Model>>()
            });

            data.push(RoleQueryReadModelDto {
                id: role.id,
                name: role.name,
                description: role.description,
                permissions,
                created_at: role.created_at,
                updated_at: role.updated_at,
            });
        }

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
