use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Community {
    pub id: Option<String>,
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

impl Community {
    pub fn get_id(&self) -> anyhow::Result<String> {
        self.id
            .clone()
            .ok_or_else(|| anyhow::anyhow!("小区ID不能为空"))
    }

    pub fn get_name(&self) -> anyhow::Result<String> {
        self.name
            .clone()
            .ok_or_else(|| anyhow::anyhow!("小区名称不能为空"))
    }

    pub fn get_address(&self) -> anyhow::Result<String> {
        self.address
            .clone()
            .ok_or_else(|| anyhow::anyhow!("小区地址不能为空"))
    }

    pub fn get_city(&self) -> anyhow::Result<String> {
        self.city
            .clone()
            .ok_or_else(|| anyhow::anyhow!("城市不能为空"))
    }

    pub fn get_location(&self) -> anyhow::Result<String> {
        self.location
            .clone()
            .ok_or_else(|| anyhow::anyhow!("位置不能为空"))
    }

    pub fn get_year_built(&self) -> anyhow::Result<u16> {
        self.year_built
            .ok_or_else(|| anyhow::anyhow!("小区年限不能为空"))
    }

    pub fn get_community_type(&self) -> anyhow::Result<String> {
        self.community_type
            .clone()
            .ok_or_else(|| anyhow::anyhow!("小区类型不能为空"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stairs {
    // 梯
    stairs: String,
    // 户
    rooms: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    pub terrace: Option<i32>,
    // 阁楼
    pub balcony: Option<i32>,
}
