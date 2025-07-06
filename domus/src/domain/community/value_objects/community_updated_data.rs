use crate::domain::community::events::community_updated::CommunityUpdatedEvent;

pub struct CommunityUpdateData {
    pub community_id: String,
    // 小区名称
    pub name: Option<String>,
    // 小区地址
    pub address: Option<String>,
    // 城市
    pub city: Option<String>,
    // 小区年限
    pub year_built: Option<u16>,
    // 小区类型
    pub community_type: Option<String>,
    // 小区描述
    pub description: Option<String>,
    // 小区图片
    pub image: Option<String>,
    // 位置
    pub location_0: Option<f64>,
    pub location_1: Option<f64>,
}

impl CommunityUpdateData {
    pub fn to_event(&self) -> CommunityUpdatedEvent {
        CommunityUpdatedEvent {
            community_id: self.community_id.clone(),
            name: self.name.clone(),
            address: self.address.clone(),
            city: self.city.clone(),
            year_built: self.year_built.clone(),
            community_type: self.community_type.clone(),
            description: self.description.clone(),
            image: self.image.clone(),
            location_0: self.location_0,
            location_1: self.location_1,
        }
    }
}
