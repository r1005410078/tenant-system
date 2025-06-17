use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserRegisteredEvent {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: Vec<String>,
    pub account_status: String,
}
