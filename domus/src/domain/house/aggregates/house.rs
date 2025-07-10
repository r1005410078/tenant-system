use sea_orm::prelude::DateTimeUtc;

use crate::domain::house::{
    events::{
        house::HouseEvent, house_deleted::HouseDeletedEvent, house_published::HousePublishedEvent,
        house_unpublished::HouseUnpublishedEvent,
    },
    value_objects::house::House,
};

pub struct HouseAggregate {
    pub house_id: String,
    // 小区id
    pub community_id: String,
    // 门牌号
    pub door_number: Option<String>,
    // 上架时间
    pub publish_at: Option<DateTimeUtc>,
    // 下架时间
    pub unpublish_at: Option<DateTimeUtc>,
    // 是否删除
    pub deleted_at: Option<DateTimeUtc>,
}

impl HouseAggregate {
    pub fn new(house_id: String, community_id: String, door_number: Option<String>) -> Self {
        Self {
            house_id,
            community_id,
            door_number,
            deleted_at: None,
            unpublish_at: None,
            publish_at: Some(chrono::Utc::now()),
        }
    }

    // 创建房源
    pub fn create(mut house: House) -> anyhow::Result<(HouseAggregate, HouseEvent)> {
        let house_id = uuid::Uuid::new_v4().to_string();

        house.validate()?;
        house.id = Some(house_id.clone());

        Ok((
            HouseAggregate::new(
                house_id.clone(),
                house.community_id.clone().unwrap(),
                house.door_number.as_ref().map(|d| d.to_string()),
            ),
            HouseEvent::Created(house.clone()),
        ))
    }

    // 删除房源
    pub fn delete(&mut self) -> HouseEvent {
        self.deleted_at = Some(chrono::Utc::now());
        HouseEvent::Deleted(HouseDeletedEvent::new(self.house_id.clone()))
    }

    // 更新房源
    pub fn update(&mut self, house: &House) -> anyhow::Result<Vec<HouseEvent>> {
        let mut events = Vec::new();

        if self.is_deleted() {
            return Err(anyhow::anyhow!("house is deleted"));
        }

        if self.is_unpublished() {
            return Err(anyhow::anyhow!("house is offline"));
        }

        // 处理事件
        if house.external_sync == Some("published".to_string()) {
            events.push(self.publish()?)
        }

        if house.external_sync == Some("unpublished".to_string()) {
            events.push(self.unpublish(house.remark.clone().unwrap_or_default().as_str())?)
        }

        events.push(HouseEvent::Updated(house.clone()));

        Ok(events)
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
