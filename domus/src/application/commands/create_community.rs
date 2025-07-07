use serde::Deserialize;

use crate::domain::{
    community::value_objects::community_created_data::CommunityCreateData,
    house::value_objects::house::Community,
};

#[derive(Debug, Deserialize)]
pub struct CreateCommunityCommand {
    // 小区名称
    pub name: String,
    // 位置id
    pub location_id: Option<String>,
    // 小区地址
    pub address: String,
    // 城市
    pub city: String,
    // 小区年限
    pub year_built: Option<u16>,
    // 小区类型
    pub community_type: String,
    // 小区描述
    pub description: Option<String>,
    // 小区图片
    pub image: Option<String>,
    // 位置
    pub location_0: Option<f64>,
    pub location_1: Option<f64>,
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
            location_id: self.location_id.clone(),
            location_0: self.location_0,
            location_1: self.location_1,
        }
    }

    pub fn from(community: &Community) -> anyhow::Result<Self> {
        Ok(CreateCommunityCommand {
            location_id: community.location_id.clone(),
            // 小区名称
            name: community.get_name()?,
            // 小区地址
            address: community.get_address()?,
            // 城市
            city: community.get_city()?,
            // 小区年限
            year_built: community.year_built.clone(),
            // 小区类型
            community_type: community.get_community_type()?,
            // 小区描述
            description: community.description.clone(),
            // 小区图片
            image: community.image.clone(),
            // 位置
            location_0: community.location_0,
            location_1: community.location_1,
        })
    }
}
