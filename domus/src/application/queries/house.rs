use std::sync::Arc;

use crate::{
    domain::house::value_objects::house::House,
    infrastructure::{
        dto::house_data_dto::HouseDataDto,
        entitiy::{community_query, house_query, owner_query},
    },
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    Condition, DbConn, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
};
use sea_orm::{ColumnTrait, RelationTrait};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

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
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }
    // 查询房源列表
    pub async fn find_all(
        &self,
        params: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<HouseDataDto>> {
        let condition = Condition::all();
        // 分页逻辑
        let page = params.page.max(1);
        let page_size = params.page_size.min(100).max(1);
        let paginator = house_query::Entity::find()
            .filter(condition.clone())
            .join(
                JoinType::LeftJoin,
                house_query::Relation::CommunityQuery.def(),
            )
            .join(JoinType::LeftJoin, house_query::Relation::OwnerQuery.def())
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
}
