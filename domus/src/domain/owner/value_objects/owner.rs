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
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_none() {
            return Err(anyhow::anyhow!("业主姓名不能为空"));
        }

        if self.phone.is_none() {
            return Err(anyhow::anyhow!("业主电话不能为空"));
        }
        Ok(())
    }
}
