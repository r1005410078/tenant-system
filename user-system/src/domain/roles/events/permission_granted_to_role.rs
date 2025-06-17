use serde::Deserialize;

#[derive(Clone)]
pub struct PermissionGrantedToRoleEvent {
    // 角色id
    role_id: String,
    // 权限
    permission: Vec<Permission>,
}

impl PermissionGrantedToRoleEvent {
    pub fn new(role_id: String, permission: Vec<Permission>) -> Self {
        Self {
            role_id,
            permission,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Permission {
    // 资源
    pub resouce: String,
    // 权限
    pub permission: String,
}
