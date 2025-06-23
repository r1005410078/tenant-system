use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct PermissionGrantedToRoleEvent {
    // 角色id
    pub role_id: String,
    // 权限
    pub permissions: Vec<Permission>,
}

impl PermissionGrantedToRoleEvent {
    pub fn new(role_id: String, permissions: Vec<Permission>) -> Self {
        Self {
            role_id,
            permissions,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Permission {
    // 资源
    pub resouce: String,
    // 权限
    pub permission: String,
}
