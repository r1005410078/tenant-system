use serde::Serialize;

use crate::domain::roles::events::permission_granted_to_role::Permission;

#[derive(Debug, Clone, Serialize)]
pub struct RoleCreatedEvent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<Permission>>,
}

impl RoleCreatedEvent {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        permissions: Option<Vec<Permission>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            permissions,
        }
    }
}
