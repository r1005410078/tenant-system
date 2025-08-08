use std::sync::Arc;

use crate::{
    domain::house::value_objects::house::ApartmentType,
    infrastructure::{
        dto::public_house_data_dto::PublicHouseDataDto,
        entitiy::{community_query, house_query, owner_query},
    },
};
use sea_orm::{
    prelude::DateTimeUtc, Condition, DbConn, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use sea_orm::{ColumnTrait, RelationTrait};
use serde::Deserialize;
use serde::Serialize;
use shared_dto::table_data::TableDataResponse;

pub struct PublicHouseQueryService {
    pool: Arc<DbConn>,
}

impl PublicHouseQueryService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }

    // 查询房源列表
    pub async fn find_all(
        &self,
        params: PubclicHouseRequest,
    ) -> anyhow::Result<TableDataResponse<PublicHouseDataDto>> {
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
        let page_size = params.page_size.min(10000).max(1);
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
            .map(|(house, community, owner)| PublicHouseDataDto::new(house, community, owner))
            .collect::<Vec<PublicHouseDataDto>>();

        Ok(TableDataResponse::new(data, total as u64))
    }

    // 根据id查询
    pub async fn find_by_id(&self, id: &str) -> Option<PublicHouseDataDto> {
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
            Ok(Some((house, community, owner))) => {
                Some(PublicHouseDataDto::new(house, community, owner))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct PubclicHouseRequest {
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
