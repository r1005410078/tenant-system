use crate::domain::owner::{
    events::{
        owner_created::OwnerCreatedEvent, owner_deleted::OwnerDeletedEvent,
        owner_updated::OwnerUpdatedEvent,
    },
    value_objects::owner::HouseOwner,
};

pub struct OwnerAggregate {
    pub owner_id: String,
    pub name: String,
    pub id_card: Option<String>,
    pub phone: String,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl OwnerAggregate {
    pub fn new(owner_id: String, name: String, phone: String, id_card: Option<String>) -> Self {
        Self {
            owner_id,
            name,
            phone,
            id_card,
            deleted_at: None,
        }
    }

    pub fn create(data: &HouseOwner) -> anyhow::Result<(OwnerAggregate, OwnerCreatedEvent)> {
        let owner_id = uuid::Uuid::new_v4().to_string();
        let name = data
            .name
            .clone()
            .ok_or(anyhow::anyhow!("Name is required"))?;

        let phone = data
            .phone
            .clone()
            .ok_or(anyhow::anyhow!("Phone is required"))?;

        let owner = OwnerAggregate::new(
            owner_id.clone(),
            name.clone(),
            phone.clone(),
            data.id_card.clone(),
        );

        let event = OwnerCreatedEvent {
            id: owner_id.clone(),
            name: name.clone(),
            phone: phone.clone(),
            id_card_images: data.id_card_images.clone(),
            description: data.description.clone(),
        };

        Ok((owner, event))
    }

    // 更新
    pub fn update(&mut self, data: &HouseOwner) -> anyhow::Result<OwnerUpdatedEvent> {
        if self.is_deleted() {
            return Err(anyhow::anyhow!("Cannot update a deleted owner"));
        }

        if let Some(name) = &data.name {
            self.name = name.clone();
        }
        if let Some(id_card) = &data.id_card {
            self.id_card = Some(id_card.clone());
        }

        Ok(OwnerUpdatedEvent {
            id: self.owner_id.clone(),
            name: Some(self.name.clone()),
            phone: data.phone.clone(),
            id_card: self.id_card.clone(),
            id_card_images: data.id_card_images.clone(),
            description: data.description.clone(),
        })
    }

    pub fn delete(&mut self) -> OwnerDeletedEvent {
        let deleted_at = chrono::Utc::now();
        self.deleted_at = Some(deleted_at);
        OwnerDeletedEvent::new(self.owner_id.clone(), deleted_at)
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
