pub struct OwnerCreatedEvent {
    // 业主ID
    pub id: String,
    // 业主姓名
    pub name: String,
    // 业主电话
    pub phone: String,
    // 业主身份证照片
    pub id_card_images: Option<Vec<String>>,
    // 业主情况
    pub description: Option<String>,
}
