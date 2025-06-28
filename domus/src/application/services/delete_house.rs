use crate::application::commands::delete_house::{DeleteHouseCommand, DeleteHouseCommandHandler};

pub struct DeleteHouseService {
    delete_house_command_handler: DeleteHouseCommandHandler,
}

impl DeleteHouseService {
    pub fn new(delete_house_command_handler: DeleteHouseCommandHandler) -> Self {
        Self {
            delete_house_command_handler,
        }
    }

    pub async fn execute(&self, house_id: String) -> anyhow::Result<()> {
        self.delete_house_command_handler
            .handle(DeleteHouseCommand::new(house_id))
            .await
    }
}
