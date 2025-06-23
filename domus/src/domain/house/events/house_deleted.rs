use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HouseDeletedEvent {
    pub id: String,
}

impl HouseDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
