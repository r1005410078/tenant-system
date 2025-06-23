use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::role_query_repository::RoleQueryRepository,
    domain::roles::events::{
        permission_granted_to_role::PermissionGrantedToRoleEvent, role_created::RoleCreatedEvent,
        role_deleted::RoleDeletedEvent, role_updated::RoleUpdatedEvent,
    },
    infrastructure::dtos::role_query_dto::RoleQueryReadModelDto,
};
use std::sync::Arc;

pub struct RoleQueryService {
    role_query_repository: Arc<dyn RoleQueryRepository>,
}

impl RoleQueryService {
    pub fn new(role_query_repository: Arc<dyn RoleQueryRepository>) -> Self {
        Self {
            role_query_repository,
        }
    }

    // 创建角色
    pub async fn create(&self, role: RoleCreatedEvent) -> anyhow::Result<()> {
        self.role_query_repository
            .create(RoleQueryReadModelDto::new(
                role.id,
                role.name,
                role.description,
                role.permissions,
            ))
            .await
    }

    // 更新角色
    pub async fn update(&self, role: RoleUpdatedEvent) -> anyhow::Result<()> {
        self.role_query_repository
            .update(RoleQueryReadModelDto::new(
                role.id,
                role.name,
                role.description,
                role.permissions,
            ))
            .await
    }

    // 删除角色
    pub async fn delete(&self, event: RoleDeletedEvent) -> anyhow::Result<()> {
        self.role_query_repository.delete(&event.id).await
    }
    // 查询角色
    pub async fn find(&self, role_id: &str) -> anyhow::Result<RoleQueryReadModelDto> {
        self.role_query_repository.find(role_id).await
    }

    // 查询角色列表
    pub async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<RoleQueryReadModelDto>> {
        self.role_query_repository
            .find_all(table_data_request)
            .await
    }

    // 绑定权限
    pub async fn bind_permissions(
        &self,
        event: &PermissionGrantedToRoleEvent,
    ) -> anyhow::Result<()> {
        self.role_query_repository
            .bind_permissions(&event.role_id, event.permissions.clone())
            .await
    }
}
