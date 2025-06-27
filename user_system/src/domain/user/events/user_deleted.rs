use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserDeletedEvent {
    pub id: String,
}

impl UserDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
