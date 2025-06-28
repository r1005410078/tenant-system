use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HouseOwner {
    // 业主ID
    pub id: Option<String>,
    // 业主姓名
    pub name: Option<String>,
    // 业主电话
    pub phone: Option<String>,
    // 业主身份证号
    pub id_card: Option<String>,
    // 业主身份证照片
    pub id_card_images: Option<Vec<String>>,
    // 业主情况
    pub description: Option<String>,
}

impl HouseOwner {
    pub fn get_name(&self) -> anyhow::Result<String> {
        self.name
            .clone()
            .ok_or_else(|| anyhow::anyhow!("业主姓名不能为空"))
    }

    pub fn get_phone(&self) -> anyhow::Result<String> {
        self.phone
            .clone()
            .ok_or_else(|| anyhow::anyhow!("业主电话不能为空"))
    }
}
