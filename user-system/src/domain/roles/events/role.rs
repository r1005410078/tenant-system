use crate::domain::roles::events::{
    permission_granted_to_role::PermissionGrantedToRoleEvent, role_created::RoleCreatedEvent,
    role_deleted::RoleDeletedEvent, role_updated::RoleUpdatedEvent,
};

#[derive(Debug, Clone)]
pub enum RoleEvent {
    RoleCreated(RoleCreatedEvent),
    RoleUpdated(RoleUpdatedEvent),
    RoleDeleted(RoleDeletedEvent),
    PermissionGrantedToRole(PermissionGrantedToRoleEvent),
}
