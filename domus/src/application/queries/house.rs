use std::sync::Arc;

use crate::{
    domain::house::value_objects::house::{ApartmentType, House},
    infrastructure::{
        dto::house_data_dto::{CommunityWithHouseCount, HouseDataDto},
        entitiy::{community_query, house_query, owner_query},
    },
};
use chrono::Utc;
use sea_orm::{
    prelude::DateTimeUtc,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    Condition, DbConn, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use sea_orm::{ColumnTrait, RelationTrait};
use serde::Deserialize;
use serde::Serialize;
use shared_dto::table_data::TableDataResponse;

pub struct HouseQueryService {
    pool: Arc<DbConn>,
}

impl HouseQueryService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        HouseQueryService { pool }
    }

    // 创建房源
    pub async fn create(&self, event: House) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(event.id.unwrap().clone()),
            community_id: event.community_id.map_or(NotSet, Set),
            owner_id: Set(event.owner_id),
            title: Set(event.title.clone()),
            purpose: Set(event.purpose.unwrap().clone()),
            transaction_type: Set(event.transaction_type.unwrap().clone()),
            house_status: Set(event.house_status.unwrap().clone()),
            door_number_from: Set(event.floor_range.as_ref().and_then(|f| f.door_number_from)),
            door_number_to: Set(event.floor_range.as_ref().and_then(|f| f.door_number_to)),
            building_number: Set(event.door_number.as_ref().and_then(|d| d.building_number)),
            unit_number: Set(event.door_number.as_ref().and_then(|d| d.unit_number)),
            door_number: Set(event.door_number.as_ref().and_then(|d| d.door_number)),
            room: Set(event.apartment_type.as_ref().and_then(|a| a.room)),
            hall: Set(event.apartment_type.as_ref().and_then(|a| a.hall)),
            bathroom: Set(event.apartment_type.as_ref().and_then(|a| a.bathroom)),
            kitchen: Set(event.apartment_type.as_ref().and_then(|a| a.kitchen)),
            terrace: Set(event.apartment_type.as_ref().and_then(|a| a.terrace)),
            balcony: Set(event.apartment_type.as_ref().and_then(|a| a.balcony)),
            building_area: Set(event.building_area),
            use_area: Set(event.use_area),
            floor_height: Set(event.floor_height),
            house_decoration: Set(event.house_decoration),
            sale_price: Set(event.sale_price),
            rent_price: Set(event.rent_price),
            rent_low_price: Set(event.rent_low_price),
            sale_low_price: Set(event.sale_low_price),
            down_payment: Set(event.down_payment),
            house_type: Set(event.house_type),
            house_orientation: Set(event.house_orientation),
            building_structure: Set(event.building_structure),
            building_year: Set(event.building_year),
            property_rights: Set(event.property_rights),
            property_year_limit: Set(event.property_year_limit),
            certificate_date: Set(event.certificate_date),
            handover_date: Set(event.handover_date),
            tags: Set(Some(serde_json::to_value(event.tags).unwrap())),
            car_height: Set(event.car_height),
            actual_rate: Set(event.actual_rate),
            level: Set(event.level),
            progress_depth: Set(event.progress_depth),
            door_width: Set(event.door_width),
            discount_year_limit: Set(event.discount_year_limit),
            stairs: Set(event.stairs.as_ref().and_then(|s| s.stairs.clone())),
            rooms: Set(event.stairs.as_ref().and_then(|s| s.rooms.clone())),
            view_method: Set(event.view_method),
            payment_method: Set(event.payment_method),
            property_tax: Set(event.property_tax),
            degree: Set(event.degree),
            household: Set(event.household),
            source: Set(event.source),
            delegate_number: Set(event.delegate_number),
            unique_housing: Set(event.unique_housing),
            full_payment: Set(event.full_payment),
            mortgage: Set(event.mortgage),
            urgent: Set(event.urgent),
            support: Set(event.support),
            present_state: Set(event.present_state),
            external_sync: Set(event.external_sync),
            remark: Set(event.remark),
            images: Set(Some(serde_json::to_value(event.images).unwrap())),
            created_by: Set(event.created_by),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    // 更新房源
    pub async fn update(&self, event: House) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(event.id.unwrap().clone()),
            community_id: event.community_id.map_or(NotSet, Set),
            owner_id: Set(event.owner_id),
            title: Set(event.title.clone()),
            purpose: event.purpose.map_or(NotSet, Set),
            transaction_type: event.transaction_type.map_or(NotSet, Set),
            house_status: event.house_status.map_or(NotSet, Set),
            door_number_from: Set(event.floor_range.as_ref().and_then(|f| f.door_number_from)),
            door_number_to: Set(event.floor_range.as_ref().and_then(|f| f.door_number_to)),
            building_number: Set(event.door_number.as_ref().and_then(|d| d.building_number)),
            unit_number: Set(event.door_number.as_ref().and_then(|d| d.unit_number)),
            door_number: Set(event.door_number.as_ref().and_then(|d| d.door_number)),
            room: Set(event.apartment_type.as_ref().and_then(|a| a.room)),
            hall: Set(event.apartment_type.as_ref().and_then(|a| a.hall)),
            bathroom: Set(event.apartment_type.as_ref().and_then(|a| a.bathroom)),
            kitchen: Set(event.apartment_type.as_ref().and_then(|a| a.kitchen)),
            terrace: Set(event.apartment_type.as_ref().and_then(|a| a.terrace)),
            balcony: Set(event.apartment_type.as_ref().and_then(|a| a.balcony)),
            building_area: Set(event.building_area),
            use_area: Set(event.use_area),
            floor_height: Set(event.floor_height),
            house_decoration: Set(event.house_decoration),
            sale_price: Set(event.sale_price),
            rent_price: Set(event.rent_price),
            rent_low_price: Set(event.rent_low_price),
            sale_low_price: Set(event.sale_low_price),
            down_payment: Set(event.down_payment),
            house_type: Set(event.house_type),
            house_orientation: Set(event.house_orientation),
            building_structure: Set(event.building_structure),
            building_year: Set(event.building_year),
            property_rights: Set(event.property_rights),
            property_year_limit: Set(event.property_year_limit),
            certificate_date: Set(event.certificate_date),
            handover_date: Set(event.handover_date),
            tags: Set(Some(serde_json::to_value(event.tags).unwrap())),
            car_height: Set(event.car_height),
            actual_rate: Set(event.actual_rate),
            level: Set(event.level),
            progress_depth: Set(event.progress_depth),
            door_width: Set(event.door_width),
            discount_year_limit: Set(event.discount_year_limit),
            stairs: Set(event.stairs.as_ref().and_then(|s| s.stairs.clone())),
            rooms: Set(event.stairs.as_ref().and_then(|s| s.rooms.clone())),
            view_method: Set(event.view_method),
            payment_method: Set(event.payment_method),
            property_tax: Set(event.property_tax),
            degree: Set(event.degree),
            household: Set(event.household),
            source: Set(event.source),
            delegate_number: Set(event.delegate_number),
            unique_housing: Set(event.unique_housing),
            full_payment: Set(event.full_payment),
            mortgage: Set(event.mortgage),
            urgent: Set(event.urgent),
            support: Set(event.support),
            present_state: Set(event.present_state),
            external_sync: Set(event.external_sync),
            remark: Set(event.remark),
            images: Set(Some(serde_json::to_value(event.images).unwrap())),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 删除房源
    pub async fn delete(&self, house_id: &str) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(house_id.to_string()),
            deleted_at: Set(Some(Utc::now())),
            ..Default::default()
        };

        // 假删除
        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 查询房源列表
    pub async fn find_all(
        &self,
        params: HouseRequest,
    ) -> anyhow::Result<TableDataResponse<HouseDataDto>> {
        let mut condition = Condition::all();
        if let Some(AmapBounds {
            north_east,
            south_west,
        }) = params.amap_bounds
        {
            condition = condition
                .add(community_query::Column::Lat.gte(south_west.lat))
                .add(community_query::Column::Lat.lte(north_east.lat))
                .add(community_query::Column::Lng.gte(south_west.lng))
                .add(community_query::Column::Lng.lte(north_east.lng));
        }

        if let Some(transaction_type) = params.transaction_type {
            condition = condition.add(house_query::Column::TransactionType.eq(transaction_type));
        }

        if let Some(purpose) = params.purpose {
            condition = condition.add(house_query::Column::Purpose.eq(purpose));
        }

        // 朝向
        if let Some(house_orientation) = params.house_orientation {
            condition = condition.add(house_query::Column::HouseOrientation.eq(house_orientation));
        }

        // 装修
        if let Some(house_decoration) = params.house_decoration {
            condition = condition.add(house_query::Column::HouseDecoration.eq(house_decoration));
        }

        // apartment_type
        if let Some(apartment_type) = params.apartment_type {
            if let Some(r) = apartment_type.room {
                condition = condition.add(house_query::Column::Room.eq(r));
            }
            if let Some(h) = apartment_type.hall {
                condition = condition.add(house_query::Column::Hall.eq(h));
            }
            if let Some(b) = apartment_type.bathroom {
                condition = condition.add(house_query::Column::Bathroom.eq(b));
            }
            if let Some(k) = apartment_type.kitchen {
                condition = condition.add(house_query::Column::Kitchen.eq(k));
            }
            if let Some(t) = apartment_type.terrace {
                condition = condition.add(house_query::Column::Terrace.eq(t));
            }
            if let Some(b) = apartment_type.balcony {
                condition = condition.add(house_query::Column::Balcony.eq(b));
            }
        }

        // 售价
        if let Some(price) = params.price {
            // 如果是 1000+
            if price.ends_with('+') {
                let price = price
                    .trim_end_matches('+')
                    .parse::<f32>()
                    .unwrap_or_default();
                condition = condition.add(house_query::Column::SalePrice.lte(price));
            }

            let price = price
                .split('-')
                .map(|p| p.parse::<f32>().unwrap_or_default())
                .collect::<Vec<f32>>();

            // 范围查询 1000-2000
            if price.len() == 2 {
                condition =
                    condition.add(house_query::Column::SalePrice.between(price[0], price[1]));
            }

            // 大于等于 1000
            if price.len() == 1 {
                condition = condition.add(house_query::Column::SalePrice.gte(price[0]));
            }
        }

        // 租金
        if let Some(rent) = params.rent {
            // 如果是 1000+
            if rent.ends_with('+') {
                let rent = rent
                    .trim_end_matches('+')
                    .parse::<f32>()
                    .unwrap_or_default();
                condition = condition.add(house_query::Column::RentPrice.lte(rent));
            }

            let rent = rent
                .split('-')
                .map(|p| p.parse::<f32>().unwrap_or_default())
                .collect::<Vec<f32>>();

            // 范围查询 1000-2000
            if rent.len() == 2 {
                condition = condition.add(house_query::Column::RentPrice.between(rent[0], rent[1]));
            }

            // 大于等于 1000
            if rent.len() == 1 {
                condition = condition.add(house_query::Column::SalePrice.gte(rent[0]));
            }
        }

        // 面积
        if let Some(area) = params.area {
            // 如果是 1000+
            if area.ends_with('+') {
                let area = area
                    .trim_end_matches('+')
                    .parse::<f32>()
                    .unwrap_or_default();
                condition = condition.add(house_query::Column::BuildingArea.lte(area));
            }

            let area = area
                .split('-')
                .map(|p| p.parse::<f32>().unwrap_or_default())
                .collect::<Vec<f32>>();

            // 范围查询 1000-2000
            if area.len() == 2 {
                condition =
                    condition.add(house_query::Column::BuildingArea.between(area[0], area[1]));
            }

            // 大于等于 1000
            if area.len() == 1 {
                condition = condition.add(house_query::Column::BuildingArea.gte(area[0]));
            }
        }

        // 楼层
        if let Some(floor) = params.floor {
            match floor.as_str() {
                "low" => {
                    condition = condition.add(house_query::Column::Level.lt(3));
                }
                "middle" => {
                    condition = condition.add(house_query::Column::Level.between(3, 6));
                }
                "high" => {
                    condition = condition.add(house_query::Column::Level.gt(6));
                }
                _ => {
                    // 如果不是 low, middle, high 则不做处理
                    condition = condition.add(house_query::Column::Level.eq(floor));
                }
            }
        }

        if let Some(updated_at) = params.updated_at {
            condition = condition.add(house_query::Column::UpdatedAt.gt(updated_at));
        }

        // 不排除已删除
        if !params.not_exclude_deleted.unwrap_or(false) {
            condition = condition.add(house_query::Column::DeletedAt.is_null());
        }

        // 分页逻辑
        let page = params.page.max(1);
        let page_size = params.page_size.min(100).max(1);
        let paginator = house_query::Entity::find()
            .join(
                JoinType::LeftJoin,
                house_query::Relation::CommunityQuery.def(),
            )
            .join(JoinType::LeftJoin, house_query::Relation::OwnerQuery.def())
            .filter(condition.clone())
            .order_by_desc(house_query::Column::UpdatedAt)
            .select_also(community_query::Entity)
            .select_also(owner_query::Entity)
            .paginate(self.pool.as_ref(), page_size as u64);

        let total = paginator.num_items().await?;
        let data = paginator.fetch_page(page - 1).await?;

        let data = data
            .into_iter()
            .map(|(house, community, owner)| HouseDataDto::new(house, community, owner))
            .collect::<Vec<HouseDataDto>>();

        Ok(TableDataResponse::new(data, total as u64))
    }

    // 根据id查询
    pub async fn find_by_id(&self, id: &str) -> Option<HouseDataDto> {
        let data = house_query::Entity::find()
            .filter(house_query::Column::Id.eq(id))
            .join(
                JoinType::LeftJoin,
                house_query::Relation::CommunityQuery.def(),
            )
            .join(JoinType::LeftJoin, house_query::Relation::OwnerQuery.def())
            .select_also(community_query::Entity)
            .select_also(owner_query::Entity)
            .one(self.pool.as_ref())
            .await;

        match data {
            Ok(Some((house, community, owner))) => Some(HouseDataDto::new(house, community, owner)),
            _ => None,
        }
    }

    // 发布房源
    pub async fn publish(&self, house_id: &str) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(house_id.to_string()),
            external_sync: Set(Some("all".to_string())),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 取消发布房源
    pub async fn unpublish(&self, house_id: &str) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(house_id.to_string()),
            external_sync: Set(None),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 获取所有的小区对应的房源个数
    pub async fn group_by_community(&self) -> anyhow::Result<Vec<CommunityWithHouseCount>> {
        // 需要join 小区， 小区id,房源个数
        let data = house_query::Entity::find()
            .select_only()
            .column(community_query::Column::Id)
            .column(community_query::Column::Name)
            .column(community_query::Column::Address)
            .column(community_query::Column::District)
            .column(community_query::Column::Adcode)
            .column(community_query::Column::Lat)
            .column(community_query::Column::Lng)
            .expr_as(house_query::Column::Id.count(), "house_count")
            .join(
                JoinType::LeftJoin,
                house_query::Relation::CommunityQuery.def(),
            )
            .group_by(community_query::Column::Id)
            .into_model::<CommunityWithHouseCount>()
            .all(self.pool.as_ref())
            .await?;

        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct HouseRequest {
    pub page: u64,
    pub page_size: u64,
    pub amap_bounds: Option<AmapBounds>,
    pub updated_at: Option<DateTimeUtc>,
    pub transaction_type: Option<String>,
    pub purpose: Option<String>,
    pub house_orientation: Option<String>,
    pub house_decoration: Option<String>,
    pub apartment_type: Option<ApartmentType>,
    pub price: Option<String>,
    pub rent: Option<String>,
    pub area: Option<String>,
    // "low", "middle", "high"
    pub floor: Option<String>,
    // 不排除已删除的
    pub not_exclude_deleted: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AmapBounds {
    pub north_east: Coord,
    pub south_west: Coord,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Coord {
    pub lng: f64,
    pub lat: f64,
}
