#[derive(Debug, Clone)]
pub struct UserBindedToRolesEvent {
    pub user_id: String,
    pub roles: Vec<String>,
}
