use serde::{Deserialize, Serialize};

use crate::domain::{
    community::value_objects::community_updated_data::CommunityUpdateData,
    house::value_objects::house::Community,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCommunityCommand {
    // 小区id
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

    pub fn from(community: &Community) -> Self {
        UpdateCommunityCommand {
            // 小区ID
            community_id: community.id.clone().unwrap(),
            // 小区名称
            name: community.name.clone(),
            // 小区地址
            address: community.address.clone(),
            // 城市
            city: community.city.clone(),
            // 小区年限
            year_built: community.year_built.clone(),
            // 小区类型
            community_type: community.community_type.clone(),
            // 小区描述
            description: community.description.clone(),
            // 小区图片
            image: community.image.clone(),
            // 位置
            location: community.location.clone(),
        }
    }
}
