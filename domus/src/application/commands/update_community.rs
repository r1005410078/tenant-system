use serde::{Deserialize, Serialize};

use crate::domain::community::value_objects::community_updated_data::CommunityUpdateData;

#[derive(Debug, Deserialize)]
pub struct UpdateCommunityCommand {
    pub community_id: String,
    // 小区名称
    pub name: Option<String>,
    // 小区地址
    pub address: Option<String>,
    // 城市
    pub city: Option<String>,
    // 小区年限
    pub year_built: Option<i16>,
    // 小区类型
    pub community_type: Option<String>,
    // 小区描述
    pub description: Option<String>,
    // 小区图片
    pub image: Option<String>,
    // 位置
    pub location: Option<String>,
}

impl UpdateCommunityCommand {
    pub fn to_data(&self) -> CommunityUpdateData {
        CommunityUpdateData {
            community_id: self.community_id.clone(),
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
