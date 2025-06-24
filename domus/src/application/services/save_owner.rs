use std::sync::Arc;

use crate::{
    application::commands::{
        create_owner::{CreateOwnerCommand, CreateOwnerCommandHandler},
        update_owner::{UpdateOwnerCommand, UpdateOwnerCommandHandler},
    },
    domain::owner::value_objects::owner::HouseOwner,
};

pub struct SaveOwnerService {
    pub create_owner_command_handler: Arc<CreateOwnerCommandHandler>,
    pub update_owner_command_handler: Arc<UpdateOwnerCommandHandler>,
}

impl SaveOwnerService {
    pub fn new(
        create_owner_command_handler: Arc<CreateOwnerCommandHandler>,
        update_owner_command_handler: Arc<UpdateOwnerCommandHandler>,
    ) -> Self {
        Self {
            create_owner_command_handler,
            update_owner_command_handler,
        }
    }

    pub async fn save(&self, owner: &HouseOwner) -> anyhow::Result<String> {
        if owner.id.is_some() {
            // 如果业主ID存在，则更新业主信息
            let command = UpdateOwnerCommand::from(owner);
            self.update_owner_command_handler.handle(command).await
        } else {
            // 创建业主
            let command = CreateOwnerCommand::from(owner)?;
            self.create_owner_command_handler.handle(command).await
        }
    }
}
