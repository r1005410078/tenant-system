use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    domain::house::value_objects::house::{
        ApartmentType, Community, DoorNumber, FloorRange, Stairs,
    },
    infrastructure::{
        dtos::{
            community_query_read_model_dto::CommunityQueryReadModelDto,
            owner_query_read_model_dto::OwnerQueryReadModelDto,
        },
        entitiy::{community_query, house_query, owner_query},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseQueryReadModelDto {
    pub id: String,
    // 房源标题
    pub title: Option<String>,
    // 用途
    pub purpose: String,
    // 交易类型
    pub transaction_type: String,
    // 状态
    pub house_status: String,
    // 楼层
    pub floor_range: FloorRange,

    // 门牌号结构
    pub door_number: DoorNumber,

    // 户型结构
    pub apartment_type: ApartmentType,

    /// 面积与装修
    // 建筑面积
    pub building_area: f32,
    // 使用面积
    pub use_area: Option<f32>,
    // 层高
    pub floor_height: Option<f32>,
    // 装修
    pub house_decoration: String,

    //// 销售租赁信息
    // 售价
    pub sale_price: Option<f64>,
    // 租价
    pub rent_price: Option<f64>,
    // 出租低价
    pub rent_low_price: Option<f64>,
    // 首付
    pub down_payment: Option<f64>,

    //// 房屋结构与产权
    // 房屋类型
    pub house_type: Option<String>,
    // 朝向
    pub house_orientation: Option<String>,
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

    //// 标签和特征
    // 推荐标签
    pub tags: Vec<String>,
    // 位置
    pub location: Option<String>,
    // 车位高度
    pub car_height: Option<f64>,
    // 实率
    pub actual_rate: Option<f64>,
    // 级别
    pub level: Option<String>,
    // 进深
    pub progress_depth: Option<f64>,
    // 门宽
    pub door_width: Option<f64>,

    /// 附加属性
    // 满减年限
    pub discount_year_limit: Option<String>,
    // 梯户
    pub stairs: Stairs,
    // 业主
    pub owner: Option<OwnerQueryReadModelDto>,
    // 小区
    pub community: Option<CommunityQueryReadModelDto>,
    // 出售低价
    pub sale_low_price: Option<f64>,
    // 看房方式
    pub view_method: Option<String>,
    // 付款方式
    pub payment_method: Option<String>,
    // 房源税费
    pub property_tax: Option<String>,
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

    // 创建时间
    pub created_at: DateTimeUtc,
    // 更新时间
    pub updated_at: DateTimeUtc,
}

impl HouseQueryReadModelDto {
    pub fn from_value(
        value: house_query::Model,
        community_query: Option<community_query::Model>,
        owner_query: Option<owner_query::Model>,
    ) -> Self {
        Self {
            community: community_query.map(CommunityQueryReadModelDto::from),
            owner: owner_query.map(OwnerQueryReadModelDto::from),
            // 基础信息
            id: value.id,
            title: value.title,
            purpose: value.purpose,
            transaction_type: value.transaction_type,
            house_status: value.house_status,
            door_number: DoorNumber {
                // 座栋
                building_number: value.building_number,
                // 单元
                unit_number: value.unit_number,
                // 门牌
                door_number: value.door_number,
            },

            apartment_type: ApartmentType {
                room: value.room,
                hall: value.hall,
                bathroom: value.bathroom,
                kitchen: value.kitchen,
                terrace: value.terrace,
                balcony: value.balcony,
            },

            floor_range: FloorRange {
                door_number_from: value.door_number_from,
                door_number_to: value.door_number_to,
            },

            building_area: value.building_area,
            use_area: value.use_area,
            floor_height: value.floor_height,
            house_decoration: value.house_decoration,
            sale_price: value.sale_price,
            rent_price: value.rent_price,
            rent_low_price: value.rent_low_price,
            sale_low_price: value.sale_low_price,
            down_payment: value.down_payment,
            house_type: value.house_type,
            house_orientation: value.house_orientation,
            building_structure: value.building_structure,
            building_year: value.building_year,
            property_rights: value.property_rights,
            property_year_limit: value.property_year_limit,
            certificate_date: value.certificate_date,
            handover_date: value.handover_date,
            tags: serde_json::from_value::<Vec<String>>(value.tags.unwrap_or(json!([])))
                .unwrap_or(vec![]),
            location: value.location,
            car_height: value.car_height,
            actual_rate: value.actual_rate,
            level: value.level,
            progress_depth: value.progress_depth,
            door_width: value.door_width,
            discount_year_limit: value.discount_year_limit,
            stairs: Stairs {
                stairs: value.stairs,
                rooms: value.rooms,
            },
            view_method: value.view_method,
            payment_method: value.payment_method,
            property_tax: value.property_tax,
            degree: value.degree,
            household: value.household,
            source: value.source,
            delegate_number: value.delegate_number,
            unique_housing: value.unique_housing,
            full_payment: value.full_payment,
            mortgage: value.mortgage,
            urgent: value.urgent,
            support: value.support,
            present_state: value.present_state,
            external_sync: value.external_sync,
            remark: value.remark,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
