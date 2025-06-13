use std::sync::Arc;

use crate::{
    application::repositories::user::UserAggregateRepository,
    domain::user::aggregates::user::UserAggregate, infrastructure::entitiy,
};
use sea_orm::*;

pub struct MySqlUserAggregateRepository {
    pool: Arc<DbConn>,
}

impl MySqlUserAggregateRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlUserAggregateRepository { pool }
    }
}

#[async_trait::async_trait]
impl UserAggregateRepository for MySqlUserAggregateRepository {
    async fn save<'a>(&self, user: &'a UserAggregate) -> anyhow::Result<()> {
        // 保存用户聚合
        let user = entitiy::user_aggregate::ActiveModel {
            id: Set(user.id.to_string()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            phone: Set(user.phone.clone()),
            password: Set(user.password.clone()),
            role: Set(user.role.clone()),
            account_status: Set(user.account_status.to_string()),
            register_time: Set(user.register_time),
            last_login_time: Set(user.last_login_time),
            ..Default::default()
        };

        user.insert(self.pool.as_ref()).await?;

        Ok(())
    }

    async fn find_by_username(&self, username: &str) -> Option<UserAggregate> {
        entitiy::user_aggregate::Entity::find()
            .filter(entitiy::user_aggregate::Column::Username.eq(username))
            .one(self.pool.as_ref())
            .await
            .map(|user| user.map(|user| user.into()))
            .unwrap()
    }

    // 用户是否存在
    async fn exists(&self, username: &str) -> bool {
        entitiy::user_aggregate::Entity::find()
            .filter(entitiy::user_aggregate::Column::Username.eq(username))
            .one(self.pool.as_ref())
            .await
            .is_ok()
    }
}

impl From<entitiy::user_aggregate::Model> for UserAggregate {
    fn from(model: entitiy::user_aggregate::Model) -> Self {
        UserAggregate {
            id: model.id.parse().unwrap(),
            username: model.username,
            email: model.email,
            phone: model.phone,
            password: model.password,
            role: model.role,
            account_status: model.account_status.into(),
            register_time: model.register_time,
            last_login_time: model.last_login_time,
        }
    }
}
