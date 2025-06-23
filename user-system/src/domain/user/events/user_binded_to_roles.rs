use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserBindedToRolesEvent {
    pub user_id: String,
    pub roles: Vec<String>,
}
