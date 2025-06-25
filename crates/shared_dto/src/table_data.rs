use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TableDataRequest {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TableDataResponse<T: Serialize> {
    pub list: Vec<T>,
    pub total: u64,
}

impl<T: Serialize> TableDataResponse<T> {
    pub fn new(list: Vec<T>, total: u64) -> TableDataResponse<T> {
        TableDataResponse { list, total }
    }
}
