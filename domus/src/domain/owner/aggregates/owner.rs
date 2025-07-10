use crate::domain::owner::{
    events::{owner::OwnerEvent, owner_deleted::OwnerDeletedEvent},
    value_objects::owner::HouseOwner,
};

#[derive(Debug, Clone)]
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

    pub fn create(mut data: HouseOwner) -> anyhow::Result<(OwnerAggregate, OwnerEvent)> {
        data.validate()?;

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

        data.id = Some(owner_id);
        Ok((owner, OwnerEvent::Created(data.clone())))
    }

    // 更新
    pub fn update(&mut self, data: &HouseOwner) -> anyhow::Result<OwnerEvent> {
        if self.is_deleted() {
            return Err(anyhow::anyhow!("Cannot update a deleted owner"));
        }

        if let Some(name) = &data.name {
            self.name = name.clone();
        }

        if let Some(id_card) = &data.id_card {
            self.id_card = Some(id_card.clone());
        }

        if let Some(phone) = &data.phone {
            self.phone = phone.clone();
        }

        Ok(OwnerEvent::Updated(data.clone()))
    }

    pub fn delete(&mut self) -> OwnerEvent {
        let deleted_at = chrono::Utc::now();
        self.deleted_at = Some(deleted_at);

        OwnerEvent::Deleted(OwnerDeletedEvent::new(self.owner_id.clone(), deleted_at))
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
