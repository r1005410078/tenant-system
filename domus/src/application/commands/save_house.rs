use serde::Deserialize;

use crate::domain::house::value_objects::house::House;

#[derive(Debug, Clone, Deserialize)]
pub struct SaveHouseCommand(House);

impl SaveHouseCommand {
    pub fn new(house: House) -> Self {
        Self(house)
    }

    pub fn into_inner(&self) -> House {
        self.0.clone()
    }
}
