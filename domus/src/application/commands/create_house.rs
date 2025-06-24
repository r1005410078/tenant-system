use serde::Deserialize;

use crate::domain::{
    house::value_objects::{
        house::{ApartmentType, Community, DoorNumber, FloorRange, Stairs},
        house_create_data::HouseCreateData,
    },
    owner::value_objects::owner::HouseOwner,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateHouseCommand {
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
    pub use_area: Option<f64>,
    // 层高
    pub floor_height: Option<f64>,
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
    pub building_year: Option<u32>,
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
    pub discount_year_limit: String,
    // 梯户
    pub stairs: Stairs,
    // 业主
    pub owner: Option<HouseOwner>,
    // 小区
    pub community: Option<Community>,
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
}

impl CreateHouseCommand {
    pub fn to_data(&self) -> HouseCreateData {
        HouseCreateData {
            // 房源标题
            title: self.title.clone(),
            // 用途
            purpose: self.purpose.clone(),
            // 交易类型
            transaction_type: self.transaction_type.clone(),
            // 状态
            house_status: self.house_status.clone(),
            // 楼层
            floor_range: self.floor_range.clone(),
            // 门牌号
            door_number: self.door_number.clone(),
            // 户型
            apartment_type: self.apartment_type.clone(),
            // 建筑面积
            building_area: self.building_area.clone(),
            // 装修
            house_decoration: self.house_decoration.clone(),
            // 满减年限
            discount_year_limit: self.discount_year_limit.clone(),
            // 梯户
            stairs: self.stairs.clone(),

            // 业主
            owner: self.owner.clone(),
            // 小区
            community: self.community.clone(),
            // 位置
            location: self.location.clone(),
            // 推荐标签
            tags: self.tags.clone(),
            // 车位高度
            car_height: self.car_height.clone(),
            // 实率
            actual_rate: self.actual_rate.clone(),
            // 级别
            level: self.level.clone(),
            // 层高
            floor_height: self.floor_height.clone(),
            // 进深
            progress_depth: self.progress_depth.clone(),
            // 门宽
            door_width: self.door_width.clone(),

            // 使用面积
            use_area: self.use_area.clone(),
            // 售价
            sale_price: self.sale_price.clone(),
            // 租价
            rent_price: self.rent_price.clone(),
            // 出租低价
            rent_low_price: self.rent_low_price.clone(),
            // 首付
            down_payment: self.down_payment.clone(),
            // 出售低价
            sale_low_price: self.sale_low_price.clone(),
            // 房屋类型
            house_type: self.house_type.clone(),
            // 朝向
            house_orientation: self.house_orientation.clone(),

            // 看房方式
            view_method: self.view_method.clone(),
            // 付款方式
            payment_method: self.payment_method.clone(),
            // 房源税费
            property_tax: self.property_tax.clone(),
            // 建筑结构
            building_structure: self.building_structure.clone(),
            // 建筑年代
            building_year: self.building_year.clone(),
            // 产权性质
            property_rights: self.property_rights.clone(),
            // 产权年限
            property_year_limit: self.property_year_limit.clone(),
            // 产证日期
            certificate_date: self.certificate_date.clone(),
            // 交房日期
            handover_date: self.handover_date.clone(),
            // 学位
            degree: self.degree.clone(),
            // 户口
            household: self.household.clone(),
            // 来源
            source: self.source.clone(),
            // 委托编号
            delegate_number: self.delegate_number.clone(),
            // 唯一住房
            unique_housing: self.unique_housing.clone(),
            // 全款
            full_payment: self.full_payment.clone(),
            // 抵押
            mortgage: self.mortgage.clone(),
            // 急切
            urgent: self.urgent.clone(),
            // 配套
            support: self.support.clone(),
            // 现状
            present_state: self.present_state.clone(),
            // 外网同步
            external_sync: self.external_sync.clone(),
            // 备注
            remark: self.remark.clone(),
        }
    }
}
