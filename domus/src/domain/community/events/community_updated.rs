use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommunityUpdatedEvent {
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
