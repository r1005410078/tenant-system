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
    //  小区id
    pub location_id: Option<String>,
    // 位置
    pub location_0: Option<f64>,
    pub location_1: Option<f64>,
}

impl Community {
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
    pub stairs: Option<String>,
    // 户
    pub rooms: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DoorNumber {
    // 座栋
    pub building_number: Option<i32>,
    // 单元
    pub unit_number: Option<i32>,
    // 门牌
    pub door_number: Option<i32>,
}

impl DoorNumber {
    pub fn to_string(&self) -> String {
        format!(
            "{}-{}-{}",
            self.building_number
                .map_or("".to_string(), |v| v.to_string()),
            self.unit_number.map_or("".to_string(), |v| v.to_string()),
            self.door_number.map_or("".to_string(), |v| v.to_string())
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FloorRange {
    // 最小楼层
    pub door_number_from: Option<i32>,
    // 最大楼层
    pub door_number_to: Option<i32>,
}

impl FloorRange {
    pub fn to_string(&self) -> String {
        format!(
            "{}-{}",
            self.door_number_from
                .map_or("".to_string(), |v| v.to_string()),
            self.door_number_to
                .map_or("".to_string(), |v| v.to_string())
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApartmentType {
    // 室
    pub room: Option<i32>,
    // 厅
    pub hall: Option<i32>,
    // 卫
    pub bathroom: Option<i32>,
    // 厨
    pub kitchen: Option<i32>,
    // 阳台
    pub terrace: Option<i32>,
    // 阁楼
    pub balcony: Option<i32>,
}
