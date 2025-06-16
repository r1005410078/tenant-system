pub struct RoleUpdatedEvent {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl RoleUpdatedEvent {
    pub fn new(id: String, name: Option<String>, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}
