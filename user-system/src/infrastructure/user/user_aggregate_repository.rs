use std::sync::Arc;

use crate::{
    application::repositories::user::UserAggregateRepository,
    domain::user::aggregates::user::UserAggregate,
    infrastructure::entitiy::{self, casbin_rules},
};
use sea_orm::*;

pub struct MySqlUserAggregateRepository {
    pool: Arc<DbConn>,
}

impl MySqlUserAggregateRepository {
    pub fn new(pool: Arc<DbConn>) -> Self {
        MySqlUserAggregateRepository { pool }
    }

    // 获取用户绑定的角色列表
    async fn get_roles_by_user_id(&self, user_id: &str) -> Vec<String> {
        casbin_rules::Entity::find()
            .filter(casbin_rules::Column::V0.eq(user_id))
            .all(self.pool.as_ref())
            .await
            .unwrap()
            .into_iter()
            .map(|x| x.v1.unwrap())
            .collect::<Vec<String>>()
    }
}

#[async_trait::async_trait]
impl UserAggregateRepository for MySqlUserAggregateRepository {
    async fn create(&self, input_user: &UserAggregate) -> anyhow::Result<()> {
        // 保存用户聚合
        let user = entitiy::user_aggregate::ActiveModel {
            id: Set(input_user.id.to_string()),
            username: Set(input_user.username.clone()),
            email: Set(input_user.email.clone()),
            phone: Set(input_user.phone.clone()),
            password: Set(input_user.password.clone()),
            account_status: Set(input_user.account_status.to_string()),
            register_time: Set(input_user.register_time),
            last_login_time: Set(input_user.last_login_time),
            deleted_at: Set(input_user.deleted_at),
            ..Default::default()
        };

        for role in input_user.roles.iter() {
            // 保存用户绑定的角色
            let model = casbin_rules::ActiveModel {
                ptype: Set("g".to_string()),
                v0: Set(Some(input_user.id.to_string())),
                v1: Set(Some(role.to_string())),
                ..Default::default()
            };
            model.insert(self.pool.as_ref()).await?;
        }

        user.insert(self.pool.as_ref()).await?;
        Ok(())
    }

    async fn save(&self, input_user: &UserAggregate) -> anyhow::Result<()> {
        // 保存用户聚合
        let user = entitiy::user_aggregate::ActiveModel {
            id: Set(input_user.id.to_string()),
            username: Set(input_user.username.clone()),
            email: Set(input_user.email.clone()),
            phone: Set(input_user.phone.clone()),
            password: Set(input_user.password.clone()),
            account_status: Set(input_user.account_status.to_string()),
            register_time: Set(input_user.register_time),
            last_login_time: Set(input_user.last_login_time),
            deleted_at: Set(input_user.deleted_at),
            ..Default::default()
        };

        // 批量删除用户绑定的角色
        casbin_rules::Entity::delete_many()
            .filter(casbin_rules::Column::V0.eq(input_user.id.to_string()))
            .exec(self.pool.as_ref())
            .await?;

        // 保存用户绑定的角色
        for role in input_user.roles.iter() {
            let model = casbin_rules::ActiveModel {
                ptype: Set("g".to_string()),
                v0: Set(Some(input_user.id.to_string())),
                v1: Set(Some(role.to_string())),
                ..Default::default()
            };
            model.insert(self.pool.as_ref()).await?;
        }

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

    async fn find_by_id(&self, user_id: &str) -> anyhow::Result<UserAggregate> {
        let mut user: UserAggregate = entitiy::user_aggregate::Entity::find()
            .filter(
                Condition::all()
                    .add(entitiy::user_aggregate::Column::Id.eq(user_id))
                    .add(entitiy::user_aggregate::Column::DeletedAt.is_null()),
            )
            .one(self.pool.as_ref())
            .await?
            .ok_or(anyhow::anyhow!("user not found"))?
            .into();

        user.roles = self.get_roles_by_user_id(user_id).await;
        Ok(user)
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
            roles: vec![],
            account_status: model.account_status.into(),
            register_time: model.register_time,
            last_login_time: model.last_login_time,
            deleted_at: model.deleted_at,
        }
    }
}
