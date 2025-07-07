use serde::Serialize;

use crate::domain::{
    house::value_objects::house::{
        ApartmentType, Community, DoorNumber, FileInfo, FloorRange, Stairs,
    },
    owner::value_objects::owner::HouseOwner,
};

#[derive(Debug, Clone, Serialize)]
pub struct HouseUpdatedEvent {
    pub house_id: String,
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

    // 业主
    pub owner: Option<HouseOwner>,
    // 小区
    pub community: Option<Community>,
    // 位置
    pub location: Option<String>,
    // 推荐标签
    pub tags: Vec<String>,
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
}
