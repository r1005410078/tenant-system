use sea_orm::prelude::DateTimeUtc;

use crate::domain::house::{
    self,
    events::{
        house::HouseEvent, house_deleted::HouseDeletedEvent, house_published::HousePublishedEvent,
        house_unpublished::HouseUnpublishedEvent,
    },
    value_objects::{house_create_data::HouseCreateData, house_update_data::HouseUpdateData},
};

pub struct HouseAggregate {
    pub house_id: String,
    // 房子地址
    pub address: String,
    // 上架时间
    pub publish_at: Option<DateTimeUtc>,
    // 下架时间
    pub unpublish_at: Option<DateTimeUtc>,
    // 是否删除
    pub deleted_at: Option<DateTimeUtc>,
}

impl HouseAggregate {
    pub fn new(house_id: String, address: String) -> Self {
        Self {
            house_id,
            address,
            publish_at: Some(chrono::Utc::now()),
            deleted_at: None,
            unpublish_at: None,
        }
    }

    // 创建房源
    pub fn create(new_house: &HouseCreateData) -> (HouseAggregate, HouseEvent) {
        let house_id = uuid::Uuid::new_v4().to_string();

        (
            HouseAggregate::new(house_id.clone(), new_house.get_address()),
            HouseEvent::Created(new_house.to_event(house_id)),
        )
    }

    // 删除房源
    pub fn delete(&mut self) -> HouseEvent {
        self.deleted_at = Some(chrono::Utc::now());
        HouseEvent::Deleted(HouseDeletedEvent::new(self.house_id.clone()))
    }

    // 更新房源
    pub fn update(&mut self, new_house: &HouseUpdateData) -> anyhow::Result<HouseEvent> {
        if self.is_deleted() {
            return Err(anyhow::anyhow!("house is deleted"));
        }

        if self.is_unpublished() {
            return Err(anyhow::anyhow!("house is offline"));
        }

        self.address = new_house.get_address();
        Ok(HouseEvent::Updated(new_house.to_event()))
    }

    // 上架
    pub fn publish(&mut self) -> anyhow::Result<HouseEvent> {
        if self.is_deleted() {
            return Err(anyhow::anyhow!("house is deleted"));
        }
        let publish_at = chrono::Utc::now();
        self.publish_at = Some(publish_at);
        Ok(HouseEvent::Published(HousePublishedEvent::new(
            self.house_id.clone(),
            publish_at,
        )))
    }

    // 下架
    pub fn unpublish(&mut self, description: &str) -> anyhow::Result<HouseEvent> {
        if self.is_deleted() {
            return Err(anyhow::anyhow!("house is deleted"));
        }

        let unpublish_at = chrono::Utc::now();
        self.unpublish_at = Some(unpublish_at);

        Ok(HouseEvent::Unpublished(HouseUnpublishedEvent::new(
            self.house_id.clone(),
            unpublish_at,
            description,
        )))
    }

    // 是否下架了
    pub fn is_unpublished(&self) -> bool {
        self.unpublish_at > self.publish_at
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
