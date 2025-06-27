use serde::Serialize;

use crate::domain::user::events::{
    user_binded_to_roles::UserBindedToRolesEvent, user_deleted::UserDeletedEvent,
    user_registered::UserRegisteredEvent, user_updated::UserUpdatedEvent,
};

#[derive(Debug, Clone, Serialize)]
pub enum UserEvent {
    UserRegistered(UserRegisteredEvent),
    UserUpdated(UserUpdatedEvent),
    UserBindedToRoles(UserBindedToRolesEvent),
    UserDeleted(UserDeletedEvent),
}
