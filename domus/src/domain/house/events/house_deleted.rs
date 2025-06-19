#[derive(Debug, Clone)]
pub struct HouseDeletedEvent {
    pub id: String,
}

impl HouseDeletedEvent {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
