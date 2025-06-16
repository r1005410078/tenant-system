#[derive(Debug, Clone)]
pub struct RoleCreatedEvent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

impl RoleCreatedEvent {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}
