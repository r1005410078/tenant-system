#[derive(Debug, Clone)]
pub struct RoleDeletedEvent {
    pub id: String,
}

impl RoleDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
