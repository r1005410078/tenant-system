use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RoleDeletedEvent {
    pub id: String,
}

impl RoleDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
