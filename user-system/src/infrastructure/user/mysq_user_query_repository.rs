use std::sync::Arc;

use sea_orm::{
    ActiveValue::Set, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use shared_dto::table_data::{TableDataRequest, TableDataResponse};

use crate::{
    application::repositories::user_query_repository::UserQueryRepository,
    infrastructure::{
        dtos::user_login_history_dto::{self, UserLoginHistoryDto},
        entitiy::{self, user_login_history},
    },
};

pub struct MysqlUserQueryRepository {
    pool: Arc<DbConn>,
}

impl MysqlUserQueryRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserQueryRepository for MysqlUserQueryRepository {
    // 查询所以用户登录历史
    async fn get_user_login_history(
        &self,
        user_id: &str,
        table_data_request: &TableDataRequest,
    ) -> anyhow::Result<TableDataResponse<UserLoginHistoryDto>> {
        let models = user_login_history::Entity::find()
            .filter(user_login_history::Column::UserId.eq(user_id))
            .order_by_desc(user_login_history::Column::LoginAt)
            .offset((table_data_request.page - 1) * table_data_request.page_size)
            .limit(table_data_request.page_size)
            .all(self.pool.as_ref())
            .await?;

        let total = user_login_history::Entity::find()
            .filter(user_login_history::Column::UserId.eq(user_id))
            .count(self.pool.as_ref())
            .await?;

        Ok(TableDataResponse {
            total,
            data: models.iter().map(|model| model.clone().into()).collect(),
        })
    }

    // 保持用户登录历史
    async fn save_user_login_history(
        &self,
        user_login_history: UserLoginHistoryDto,
    ) -> anyhow::Result<()> {
        let model = user_login_history::ActiveModel {
            user_id: Set(user_login_history.user_id),
            username: Set(user_login_history.username),
            status: Set(user_login_history.status),
            login_at: Set(user_login_history.login_at),
            ..Default::default()
        };

        user_login_history::Entity::insert(model)
            .exec(self.pool.as_ref())
            .await?;

        Ok(())
    }
}
