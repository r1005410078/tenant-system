pub struct CommunityAggregate {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub deleted_at: Option<String>,
}

impl CommunityAggregate {
    pub fn new(id: i32, name: String, address: String) -> CommunityAggregate {
        CommunityAggregate {
            id,
            name,
            address,
            deleted_at: None,
        }
    }

    pub fn delete(&mut self) {
        self.deleted_at = Some(String::from("now()"));
    }

    pub fn update(&mut self, name: String, address: String) {
        self.name = name;
        self.address = address;
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
