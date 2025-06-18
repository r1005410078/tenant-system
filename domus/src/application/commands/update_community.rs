pub struct CreateCommunityCommand {
    pub id: i32,
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub year_built: Option<i16>,
    pub community_type: Option<String>,
    pub description: Option<String>,
}
