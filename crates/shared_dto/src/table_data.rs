use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TableDataRequest {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TableDataResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: u64,
}
