use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    domain::roles::events::permission_granted_to_role::Permission,
    infrastructure::dtos::role_query_dto::RoleQueryReadModelDto,
};

#[async_trait::async_trait]
pub trait RoleQueryRepository: Send + Sync {
    // 创建角色
    async fn create(&self, role: RoleQueryReadModelDto) -> anyhow::Result<()>;
    // 更新角色
    async fn update(&self, role: RoleQueryReadModelDto) -> anyhow::Result<()>;
    // 删除角色
    async fn delete(&self, role_id: &str) -> anyhow::Result<()>;
    // 查询角色
    async fn find(&self, role_id: &str) -> anyhow::Result<RoleQueryReadModelDto>;
    // 查询角色列表
    async fn find_all(
        &self,
        table_data_request: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<RoleQueryReadModelDto>>;

    // 绑定角色权限
    async fn bind_permissions(
        &self,
        role_id: &str,
        permissions: Vec<Permission>,
    ) -> anyhow::Result<()>;
}
