#[derive(Debug, Clone)]
pub struct UserDeletedEvent {
    pub id: String,
}

impl UserDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
