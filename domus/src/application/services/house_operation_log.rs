use std::sync::Arc;

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, Order, QueryFilter, QueryOrder};
use shared_utils::diff_values::diff_structs;

use crate::domain::house::value_objects::house::House;
use crate::infrastructure::entitiy::{house_operation_log, house_query};

pub struct HouseOperationLogService {
    db: Arc<DbConn>,
}

impl HouseOperationLogService {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

impl HouseOperationLogService {
    pub async fn save_record(&self, data: HouseOperationLogDto) -> anyhow::Result<()> {
        let after = serde_json::to_value(data.operation_content.clone())?;
        let house_id = data
            .operation_content
            .id
            .ok_or(anyhow::anyhow!("House id is required"))?;

        let mut model = house_operation_log::ActiveModel {
            house_id: Set(house_id.clone()),
            operation_type: Set(data.operation_type),
            operator_id: Set(data.operator_id),
            ip_address: Set(data.ip_address),
            user_agent: Set(data.user_agent),
            ..Default::default()
        };

        if data.operation_type == 2 {
            let last = house_query::Entity::find()
                .filter(house_query::Column::Id.eq(house_id.clone()))
                .order_by_desc(house_query::Column::UpdatedAt)
                .one(self.db.as_ref())
                .await?;

            let before = match last {
                Some(last) => serde_json::to_value(last)?,
                None => r#"{}"#.to_string().into(),
            };

            let diff_value = diff_structs(&before, &after);
            model.operation_content = Set(Some(diff_value));
        } else {
            model.operation_content = NotSet;
        }

        model.insert(self.db.as_ref()).await?;

        Ok(())
    }

    pub async fn list(&self, house_id: &str) -> anyhow::Result<Vec<house_operation_log::Model>> {
        house_operation_log::Entity::find()
            .filter(house_operation_log::Column::HouseId.eq(house_id))
            .order_by(house_operation_log::Column::CreatedAt, Order::Asc)
            .all(self.db.as_ref())
            .await
            .map_err(anyhow::Error::from)
    }
}

pub struct HouseOperationLogDto {
    // '操作类型：1=新增，2=修改，3=删除，4=上架，5=下架'
    pub operation_type: u8,
    pub operation_content: House,
    pub operator_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
