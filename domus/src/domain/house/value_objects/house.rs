use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::domain::{
    community::value_objects::commuity::Community, owner::value_objects::owner::HouseOwner,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseData {
    // 房源
    #[serde(flatten)]
    pub house: Option<House>,
    // 小区
    pub community: Option<Community>,
    // 所有者
    pub owner: Option<HouseOwner>,
}

impl HouseData {
    pub fn update_created_by(&mut self, created_by: String) {
        if let Some(house) = &mut self.house {
            house.created_by = Some(created_by);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    pub id: Option<String>,
    pub created_by: Option<String>,
    pub community_id: Option<String>,
    pub owner_id: Option<String>,
    // 房源标题
    pub title: Option<String>,
    // 用途
    pub purpose: Option<String>,
    // 交易类型
    pub transaction_type: Option<String>,
    // 状态
    pub house_status: Option<String>,
    // 楼层
    pub floor_range: Option<FloorRange>,
    // 门牌号
    pub door_number: Option<DoorNumber>,
    // 户型
    pub apartment_type: Option<ApartmentType>,
    // 建筑面积
    pub building_area: Option<f32>,
    // 装修
    pub house_decoration: Option<String>,
    // 满减年限
    pub discount_year_limit: Option<String>,
    // 梯户
    pub stairs: Option<Stairs>,
    // 推荐标签
    pub tags: Option<Vec<String>>,
    // 车位高度
    pub car_height: Option<f64>,
    // 实率
    pub actual_rate: Option<f64>,
    // 级别
    pub level: Option<String>,
    // 层高
    pub floor_height: Option<f32>,
    // 进深
    pub progress_depth: Option<f64>,
    // 门宽
    pub door_width: Option<f64>,

    // 使用面积
    pub use_area: Option<f32>,
    // 售价
    pub sale_price: Option<f64>,
    // 租价
    pub rent_price: Option<f64>,
    // 出租低价
    pub rent_low_price: Option<f64>,
    // 首付
    pub down_payment: Option<f64>,
    // 出售低价
    pub sale_low_price: Option<f64>,
    // 房屋类型
    pub house_type: Option<String>,
    // 朝向
    pub house_orientation: Option<String>,

    // 看房方式
    pub view_method: Option<String>,
    // 付款方式
    pub payment_method: Option<String>,
    // 房源税费
    pub property_tax: Option<String>,
    // 建筑结构
    pub building_structure: Option<String>,
    // 建筑年代
    pub building_year: Option<i32>,
    // 产权性质
    pub property_rights: Option<String>,
    // 产权年限
    pub property_year_limit: Option<String>,
    // 产证日期
    pub certificate_date: Option<String>,
    // 交房日期
    pub handover_date: Option<String>,
    // 学位
    pub degree: Option<String>,
    // 户口
    pub household: Option<String>,
    // 来源
    pub source: Option<String>,
    // 委托编号
    pub delegate_number: Option<String>,
    // 唯一住房
    pub unique_housing: Option<String>,
    // 全款
    pub full_payment: Option<String>,
    // 抵押
    pub mortgage: Option<String>,
    // 急切
    pub urgent: Option<String>,
    // 配套
    pub support: Option<String>,
    // 现状
    pub present_state: Option<String>,
    // 外网同步
    pub external_sync: Option<String>,
    // 备注
    pub remark: Option<String>,
    // 房源图片
    pub images: Option<Vec<FileInfo>>,
    // 更新时间
    pub updated_at: Option<DateTimeUtc>,
}

impl House {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.purpose.is_none() {
            return Err(anyhow::anyhow!("用途不能为空"));
        }

        if self.transaction_type.is_none() {
            return Err(anyhow::anyhow!("交易类型不能为空"));
        }

        if self.house_status.is_none() {
            return Err(anyhow::anyhow!("房源状态不能为空"));
        }
        Ok(())
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
    // 楼层
    pub floor: Option<i32>,
}

impl DoorNumber {
    pub fn to_string(&self) -> String {
        let mut addr = String::new();

        if let Some(building_number) = self.building_number {
            addr.push_str(&building_number.to_string());
            addr.push_str("幢");
        }

        if let Some(unit_number) = self.unit_number {
            if !addr.is_empty() {
                addr.push_str("-");
            }
            addr.push_str(&unit_number.to_string());
            addr.push_str("单元");
        }

        if let Some(door_number) = self.door_number {
            if !addr.is_empty() {
                addr.push_str("-");
            }
            addr.push_str(&door_number.to_string());
            addr.push_str("室");
        }

        if let Some(floor) = self.floor {
            if !addr.is_empty() {
                addr.push_str("-");
            }
            addr.push_str(&floor.to_string());
            addr.push_str("层");
        }

        addr
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FloorRange {
    // 最小楼层
    pub door_number_from: Option<i32>,
    // 最大楼层
    pub door_number_to: Option<i32>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileInfo {
    // 名称
    pub name: String,
    // 类型
    pub r#type: String,
    // 卫
    pub size: String,
    // 厨
    pub url: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::domain::house::value_objects::house::HouseData;

    #[test]
    fn test_validate() {
        let json = json!({
            "owner": {
                "name": "taosheng rong:本人",
                "description": "",
                "phone": "18626891229:本人"
            },
            "community": {
                "id": "B0FFGA9AAU",
                "name": "东安花园",
                "address": "华中路与港华路交叉口东北100米",
                "year_built": "2025-07-10T09:20:15.123Z",
                "description": "A place for new ideas",
                "images": ["https://example.com/image1.jpg", "https://example.com/image2.jpg"],
                "typecode": "residential",
                "district": "Manhattan",
                "adcode": "10001",
                "lat": 40.7128,
                "lng": -74.006
            },
            "purpose": "住宅",
            "transaction_type": "出售",
            "house_status": "有效",
            "sale_price": 12,
            "sale_low_price": 11,
            "down_payment": 2,
            "door_number": {
                "building_number": 1,
                "unit_number": 1,
                "door_number": 1
            },
            "apartment_type": {
                "room": 1,
                "bathroom": 1,
                "kitchen": 1,
                "balcony": 1
            }
        });
        let data: HouseData = serde_json::from_value(json).unwrap();

        println!("data: {:#?}", data);
    }
}
