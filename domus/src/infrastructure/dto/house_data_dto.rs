use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::{
    domain::house::value_objects::house::{ApartmentType, DoorNumber, FloorRange, House, Stairs},
    infrastructure::entitiy::{self},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseDataDto {
    // 房源
    #[serde(flatten)]
    pub house: House,
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
        let house = House {
            id: Some(house.id),
            created_by: house.created_by,
            community_id: Some(house.community_id),
            owner_id: house.owner_id,
            // 房源标题
            title: house.title,
            // 用途
            purpose: Some(house.purpose),
            // 交易类型
            transaction_type: Some(house.transaction_type),
            // 状态
            house_status: Some(house.house_status),
            // 楼层
            floor_range: Some(FloorRange {
                door_number_from: house.door_number_from,
                door_number_to: house.door_number_to,
            }),
            // 门牌号
            door_number: Some(DoorNumber {
                building_number: house.building_number,
                unit_number: house.unit_number,
                door_number: house.door_number,
                floor: house.current_floor,
            }),
            // 户型
            apartment_type: Some(ApartmentType {
                // 室
                room: house.room,
                // 厅
                hall: house.hall,
                // 卫
                bathroom: house.bathroom,
                // 厨
                kitchen: house.kitchen,
                // 阳台
                terrace: house.terrace,
                // 阁楼
                balcony: house.balcony,
            }),
            // 建筑面积
            building_area: house.building_area,
            // 装修
            house_decoration: house.house_decoration,
            // 满减年限
            discount_year_limit: house.discount_year_limit,
            // 梯户
            stairs: Some(Stairs {
                // 梯
                stairs: house.stairs,
                // 户
                rooms: house.rooms,
            }),
            // 推荐标签
            tags: house
                .tags
                .map(|tags| serde_json::from_value(tags).unwrap_or_default()),
            // 车位高度
            car_height: house.car_height,
            // 实率
            actual_rate: house.actual_rate,
            // 级别
            level: house.level,
            // 层高
            floor_height: house.floor_height,
            // 进深
            progress_depth: house.progress_depth,
            // 门宽
            door_width: house.door_width,

            // 使用面积
            use_area: house.use_area,
            // 售价
            sale_price: house.sale_price,
            // 租价
            rent_price: house.rent_price,
            // 出租低价
            rent_low_price: house.rent_low_price,
            // 首付
            down_payment: house.down_payment,
            // 出售低价
            sale_low_price: house.sale_low_price,
            // 房屋类型
            house_type: house.house_type,
            // 朝向
            house_orientation: house.house_orientation,

            // 看房方式
            view_method: house.view_method,
            // 付款方式
            payment_method: house.payment_method,
            // 房源税费
            property_tax: house.property_tax,
            // 建筑结构
            building_structure: house.building_structure,
            // 建筑年代
            building_year: house.building_year,
            // 产权性质
            property_rights: house.property_rights,
            // 产权年限
            property_year_limit: house.property_year_limit,
            // 产证日期
            certificate_date: house.certificate_date,
            // 交房日期
            handover_date: house.handover_date,
            // 学位
            degree: house.degree,
            // 户口
            household: house.household,
            // 来源
            source: house.source,
            // 委托编号
            delegate_number: house.delegate_number,
            // 唯一住房
            unique_housing: house.unique_housing,
            // 全款
            full_payment: house.full_payment,
            // 抵押
            mortgage: house.mortgage,
            // 急切
            urgent: house.urgent,
            // 配套
            support: house.support,
            // 现状
            present_state: house.present_state,
            // 外网同步
            external_sync: house.external_sync,
            // 备注
            remark: house.remark,
            // 房源图片
            images: house
                .images
                .map(|images| serde_json::from_value(images).unwrap_or_default()),
            // 更新时间
            updated_at: Some(house.updated_at),
            // 删除时间
            deleted_at: house.deleted_at,
        };

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
