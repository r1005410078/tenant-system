#[derive(Debug, Clone)]
pub struct OwnerUpdatedEvent {
    // 业主ID
    pub id: String,
    // 业主姓名
    pub name: Option<String>,
    // 业主电话
    pub phone: Option<String>,
    // 身份证号
    pub id_card: Option<String>,
    // 业主身份证照片
    pub id_card_images: Option<Vec<String>>,
    // 业主情况
    pub description: Option<String>,
}
