#[derive(Debug, Clone)]
pub struct UserUpdatedEvent {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub account_status: String,
    pub role: Vec<String>,
}
