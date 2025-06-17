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
    async fn create(&self, user: &UserAggregate) -> anyhow::Result<()> {
        // 保存用户聚合
        let user = entitiy::user_aggregate::ActiveModel {
            id: Set(user.id.to_string()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            phone: Set(user.phone.clone()),
            password: Set(user.password.clone()),
            account_status: Set(user.account_status.to_string()),
            register_time: Set(user.register_time),
            last_login_time: Set(user.last_login_time),
            deleted_at: Set(user.deleted_at),
            ..Default::default()
        };

        user.insert(self.pool.as_ref()).await?;

        Ok(())
    }

    async fn save(&self, user: &UserAggregate) -> anyhow::Result<()> {
        // 保存用户聚合
        let user = entitiy::user_aggregate::ActiveModel {
            id: Set(user.id.to_string()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            phone: Set(user.phone.clone()),
            password: Set(user.password.clone()),
            account_status: Set(user.account_status.to_string()),
            register_time: Set(user.register_time),
            last_login_time: Set(user.last_login_time),
            deleted_at: Set(user.deleted_at),
            ..Default::default()
        };

        user.update(self.pool.as_ref()).await?;
        Ok(())
    }

    async fn find_by_username(&self, username: &str) -> Option<UserAggregate> {
        entitiy::user_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::user_aggregate::Column::Username.eq(username))
                    .add(entitiy::user_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await
            .map(|user| user.map(|user| user.into()))
            .unwrap()
    }

    async fn find_by_id(&self, user_id: &str) -> Option<UserAggregate> {
        let res = entitiy::user_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::user_aggregate::Column::Id.eq(user_id))
                    .add(entitiy::user_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await;

        match res {
            Ok(data) => data.map(Into::into),
            Err(_) => None,
        }
    }

    // 用户是否存在
    async fn exists(&self, username: &str) -> anyhow::Result<bool> {
        let data = entitiy::user_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::user_aggregate::Column::Username.eq(username))
                    .add(entitiy::user_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await?;

        Ok(data.is_some())
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
            role: vec![],
            account_status: model.account_status.into(),
            register_time: model.register_time,
            last_login_time: model.last_login_time,
            deleted_at: model.deleted_at,
        }
    }
}
