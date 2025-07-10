use serde::{Deserialize, Serialize};

use crate::infrastructure::entitiy::{self};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseDataDto {
    // 房源
    #[serde(flatten)]
    pub house: entitiy::house_query::Model,
    // 小区
    pub community: Option<entitiy::community_query::Model>,
    // 所有者
    pub owner: Option<entitiy::owner_query::Model>,
}

impl HouseDataDto {
    pub fn new(
        house: entitiy::house_query::Model,
        community: Option<entitiy::community_query::Model>,
        owner: Option<entitiy::owner_query::Model>,
    ) -> Self {
        Self {
            house,
            community,
            owner,
        }
    }
}

pub struct CommunityWithHouseCount {
    // 小区
    pub id: String,
    pub name: String,
    pub address: String,
    // 个数
    pub count: Option<i64>,
}

impl CommunityWithHouseCount {
    pub fn new(community: Option<entitiy::community_query::Model>, count: Option<i64>) -> Self {
        Self { community, count }
    }
}
