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

pub struct HouseOwner {
    // 业主姓名
    pub name: String,
    // 业主电话
    pub phone: String,
    // 业主身份证照片
    pub id_card_images: Option<Vec<String>>,
    // 业主情况
    pub description: Option<String>,
}

pub struct Stairs {
    // 梯
    stairs: String,
    // 户
    rooms: String,
}

pub struct DoorNumber {
    // 座栋
    pub building_number: i32,
    // 单元
    pub unit_number: i32,
    // 门牌
    pub door_number: i32,
}

pub struct FloorRange {
    // 最小楼层
    pub door_number_from: i32,
    // 最大楼层
    pub door_number_to: i32,
}

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
