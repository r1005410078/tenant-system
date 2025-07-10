use sea_orm::FromQueryResult;
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

#[derive(Debug, Clone, FromQueryResult, Serialize, Deserialize)]
pub struct CommunityWithHouseCount {
    // 小区
    pub id: String,
    // 小区名称
    pub name: String,
    // 小区地址
    pub address: String,
    // 所属区域
    pub district: Option<String>,
    // 所属行政区划代码（如“110105”，代表朝阳区）
    pub adcode: Option<String>,
    // 小区坐标
    pub lat: f64,
    pub lng: f64,
    // 个数
    pub house_count: Option<i64>,
}
