pub struct CreateCommunityCommand {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub year_built: i16,
    pub community_type: String,
    pub description: Option<String>,
}
