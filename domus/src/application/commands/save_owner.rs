use serde::Deserialize;

use crate::domain::owner::value_objects::owner::HouseOwner;

#[derive(Debug, Clone, Deserialize)]
pub struct SaveOwnerCommand(HouseOwner);

impl SaveOwnerCommand {
    pub fn new(owner: HouseOwner) -> Self {
        SaveOwnerCommand(owner)
    }

    pub fn into_inner(&self) -> HouseOwner {
        self.0.clone()
    }
}
