use std::sync::Arc;

use crate::{
    application::repositories::house_query_repository::HouseQueryRepository,
    domain::house::events::{house_created::HouseCreatedEvent, house_updated::HouseUpdatedEvent},
    infrastructure::{
        dtos::house_query_read_model_dto::HouseQueryReadModelDto,
        entitiy::{community_query, house_query, owner_query},
    },
};
use sea_orm::RelationTrait;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    Condition, DbConn, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

pub struct MySqlHouseQueryRepository {
    pool: Arc<DbConn>,
}

impl MySqlHouseQueryRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlHouseQueryRepository { pool }
    }
}

#[async_trait::async_trait]
impl HouseQueryRepository for MySqlHouseQueryRepository {
    // 创建房源
    async fn create(&self, event: HouseCreatedEvent) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            community_id: event.community.map(|c| c.id).flatten().map_or(NotSet, Set),
            owner_id: event.owner.id.map_or(NotSet, Set),
            id: Set(event.house_id.clone()),
            title: Set(event.title.clone()),
            purpose: Set(event.purpose.clone()),
            transaction_type: Set(event.transaction_type.clone()),
            house_status: Set(event.house_status.clone()),
            door_number_from: Set(event.floor_range.door_number_from),
            door_number_to: Set(event.floor_range.door_number_to),
            building_number: Set(event.door_number.building_number),
            unit_number: Set(event.door_number.unit_number),
            door_number: Set(event.door_number.door_number),
            room: Set(event.apartment_type.room),
            hall: Set(event.apartment_type.hall),
            bathroom: Set(event.apartment_type.bathroom),
            kitchen: Set(event.apartment_type.kitchen),
            terrace: Set(event.apartment_type.terrace),
            balcony: Set(event.apartment_type.balcony),
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
            location: Set(event.location),
            car_height: Set(event.car_height),
            actual_rate: Set(event.actual_rate),
            level: Set(event.level),
            progress_depth: Set(event.progress_depth),
            door_width: Set(event.door_width),
            discount_year_limit: Set(event.discount_year_limit),
            stairs: Set(event.stairs.stairs),
            rooms: Set(event.stairs.rooms),
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
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    // 更新房源
    async fn update(&self, event: HouseUpdatedEvent) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            community_id: event.community.map(|c| c.id).flatten().map_or(NotSet, Set),
            owner_id: event.owner.map(|c| c.id).flatten().map_or(NotSet, Set),
            id: Set(event.house_id.clone()),
            title: Set(event.title.clone()),
            purpose: event.purpose.map_or(NotSet, Set),
            transaction_type: event.transaction_type.map_or(NotSet, Set),
            house_status: event.house_status.map_or(NotSet, Set),
            door_number_from: event
                .floor_range
                .clone()
                .map(|f| f.door_number_from)
                .map_or(NotSet, Set),
            door_number_to: event
                .floor_range
                .clone()
                .map(|f| f.door_number_to)
                .map_or(NotSet, Set),
            building_number: event
                .door_number
                .clone()
                .map(|d| d.building_number)
                .map_or(NotSet, Set),
            unit_number: event
                .door_number
                .clone()
                .map(|d| d.unit_number)
                .map_or(NotSet, Set),
            door_number: event
                .door_number
                .clone()
                .map(|d| d.door_number)
                .map_or(NotSet, Set),
            room: event
                .apartment_type
                .clone()
                .map(|a| a.room)
                .map_or(NotSet, Set),
            hall: event
                .apartment_type
                .clone()
                .map(|a| a.hall)
                .map_or(NotSet, Set),
            bathroom: event
                .apartment_type
                .clone()
                .map(|a| a.bathroom)
                .map_or(NotSet, Set),
            kitchen: event
                .apartment_type
                .clone()
                .map(|a| a.kitchen)
                .map_or(NotSet, Set),
            terrace: event
                .apartment_type
                .clone()
                .map(|a| a.terrace)
                .map_or(NotSet, Set),
            balcony: event
                .apartment_type
                .clone()
                .map(|a| a.balcony)
                .map_or(NotSet, Set),
            building_area: event.building_area.map_or(NotSet, Set),
            use_area: Set(event.use_area),
            floor_height: Set(event.floor_height),
            house_decoration: event.house_decoration.map_or(NotSet, Set),
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
            location: Set(event.location),
            car_height: Set(event.car_height),
            actual_rate: Set(event.actual_rate),
            level: Set(event.level),
            progress_depth: Set(event.progress_depth),
            door_width: Set(event.door_width),
            discount_year_limit: Set(event.discount_year_limit),
            stairs: event.stairs.clone().map(|s| s.stairs).map_or(NotSet, Set),
            rooms: event.stairs.clone().map(|s| s.rooms).map_or(NotSet, Set),
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
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 删除房源
    async fn delete(&self, house_id: &str) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(house_id.to_string()),
            ..Default::default()
        };

        model.delete(self.pool.as_ref()).await?;
        Ok(())
    }
    // 查询房源列表
    async fn find_all(
        &self,
        params: TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<HouseQueryReadModelDto>> {
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
            .map(|(a, b, c)| HouseQueryReadModelDto::from_value(a, b, c))
            .collect::<Vec<HouseQueryReadModelDto>>();

        Ok(TableDataResponse::new(data, total as u64))
    }

    // 发布房源
    async fn publish(&self, house_id: &str) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(house_id.to_string()),
            external_sync: Set(Some("all".to_string())),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }

    // 取消发布房源
    async fn unpublish(&self, house_id: &str) -> anyhow::Result<()> {
        let model = house_query::ActiveModel {
            id: Set(house_id.to_string()),
            external_sync: Set(None),
            ..Default::default()
        };

        model.update(self.pool.as_ref()).await?;
        Ok(())
    }
}
