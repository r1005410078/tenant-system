use crate::domain::community::events::community_created::CommunityCreatedEvent;

pub struct CommunityCreateData {
    // 小区名称
    pub name: String,
    // 小区地址
    pub address: String,
    // 城市
    pub city: String,
    // 小区年限
    pub year_built: u16,
    // 小区类型
    pub community_type: String,
    // 小区描述
    pub description: Option<String>,
    // 小区图片
    pub image: Option<String>,
    // 位置
    pub location: Option<String>,
}

impl CommunityCreateData {
    pub fn to_event(&self, community_id: &str) -> CommunityCreatedEvent {
        CommunityCreatedEvent {
            community_id: community_id.to_string(),
            name: self.name.clone(),
            address: self.address.clone(),
            city: self.city.clone(),
            year_built: self.year_built,
            community_type: self.community_type.clone(),
            description: self.description.clone(),
            image: self.image.clone(),
            location: self.location.clone(),
        }
    }
}
