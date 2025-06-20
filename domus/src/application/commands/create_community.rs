use serde::Deserialize;

use crate::domain::community::value_objects::community_created_data::CommunityCreateData;

#[derive(Debug, Deserialize)]
pub struct CreateCommunityCommand {
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

impl CreateCommunityCommand {
    pub fn to_data(&self) -> CommunityCreateData {
        CommunityCreateData {
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
