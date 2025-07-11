use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, EntityTrait, QueryFilter};

use crate::infrastructure::entitiy::house_comments;

pub struct HouseCommentService {
    pool: Arc<DbConn>,
}

impl HouseCommentService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        HouseCommentService { pool }
    }

    /// 添加房屋评论
    pub async fn add_comment(
        &self,
        admin_id: &str,
        house_id: &str,
        content: &str,
    ) -> anyhow::Result<()> {
        let model = house_comments::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            admin_id: Set(admin_id.to_string()),
            house_id: Set(house_id.to_string()),
            content: Set(content.to_string()),
            ..Default::default()
        };

        model.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    /// 更新评论
    pub async fn update_comment(
        &self,
        admin_id: &str,
        comment_id: &str,
        content: &str,
    ) -> anyhow::Result<()> {
        let model = house_comments::ActiveModel {
            content: Set(content.to_string()),
            ..Default::default()
        };

        let res = house_comments::Entity::update_many()
            .set(model)
            .filter(
                house_comments::Column::Id
                    .eq(comment_id.to_string())
                    .and(house_comments::Column::AdminId.eq(admin_id.to_string())),
            )
            .exec(self.pool.as_ref())
            .await?;

        println!("res: {:#?}", res.rows_affected);

        Ok(())
    }

    // 删除评论
    pub async fn delete_comment(&self, admin_id: &str, id: &str) -> anyhow::Result<()> {
        house_comments::Entity::delete_many()
            .filter(
                house_comments::Column::Id
                    .eq(id.to_string())
                    .and(house_comments::Column::AdminId.eq(admin_id.to_string())),
            )
            .exec(self.pool.as_ref())
            .await?;

        Ok(())
    }

    // 查询评论
    pub async fn get_comments(&self, house_id: &str) -> anyhow::Result<Vec<house_comments::Model>> {
        let model = house_comments::Entity::find()
            .filter(house_comments::Column::HouseId.eq(house_id))
            .all(self.pool.as_ref())
            .await?;

        Ok(model)
    }
}
