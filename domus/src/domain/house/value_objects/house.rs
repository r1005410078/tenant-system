#[derive(Debug, Clone)]
pub struct Community {
    pub id: Option<i32>,
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

#[derive(Debug, Clone)]
pub struct Stairs {
    // 梯
    stairs: String,
    // 户
    rooms: String,
}

#[derive(Debug, Clone)]
pub struct DoorNumber {
    // 座栋
    pub building_number: i32,
    // 单元
    pub unit_number: i32,
    // 门牌
    pub door_number: i32,
}

impl DoorNumber {
    pub fn to_string(&self) -> String {
        format!(
            "{}-{}-{}",
            self.building_number, self.unit_number, self.door_number
        )
    }
}

#[derive(Debug, Clone)]
pub struct FloorRange {
    // 最小楼层
    pub door_number_from: i32,
    // 最大楼层
    pub door_number_to: i32,
}

impl FloorRange {
    pub fn to_string(&self) -> String {
        format!("{}-{}", self.door_number_from, self.door_number_to)
    }
}

#[derive(Debug, Clone)]
pub struct ApartmentType {
    // 室
    pub room: i32,
    // 厅
    pub hall: i32,
    // 卫
    pub bathroom: i32,
    // 厨
    pub kitchen: i32,
    // 阳台
    pub terrace: i32,
    // 阁楼
    pub balcony: i32,
}
