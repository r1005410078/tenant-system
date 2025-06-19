pub struct CommunityCreatedEvent {
    pub id: i32,
    // 小区名称
    pub name: String,
    // 小区地址
    pub address: String,
    // 城市
    pub city: String,
    // 小区年限
    pub year_built: i16,
    // 小区类型
    pub community_type: String,
    // 小区描述
    pub description: Option<String>,
    // 小区图片
    pub image: Option<String>,
    // 位置
    pub location: Option<String>,
}
