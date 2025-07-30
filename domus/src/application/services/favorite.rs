use crate::application::listeners::house;
use crate::infrastructure::dto::house_data_dto::HouseDataDto;
use crate::infrastructure::entitiy::{
    community_query, favorite_categories, house_query, owner_query, user_favorites,
};
use sea_orm::{
    query, ActiveModelTrait, ColumnTrait, Condition, EntityOrSelect, EntityTrait, JoinType,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use sea_orm::{ActiveValue::Set, DbConn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavoriteCategories {
    pub id: Option<i64>,
    pub user_id: Option<String>,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserFavoriteQueryDto {
    pub user_id: Option<String>,
    pub category_id: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserFavorites {
    pub id: Option<i64>,
    pub user_id: Option<String>,
    pub house_id: String,
    pub category_id: Option<i64>,
}

pub struct FavoriteService {
    pool: Arc<DbConn>,
}

impl FavoriteService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        FavoriteService { pool }
    }
}

impl FavoriteService {
    // 给用户添加收藏夹
    pub async fn add_favorite_categories(&self, data: FavoriteCategories) -> anyhow::Result<i64> {
        let model = favorite_categories::ActiveModel {
            user_id: Set(data.user_id.unwrap()),
            name: Set(data.name),
            color: Set(data.color),
            ..Default::default()
        };

        let favorite_categories = model.insert(self.pool.as_ref()).await?;
        Ok(favorite_categories.id)
    }

    pub async fn update_favorite_categories(
        &self,
        data: FavoriteCategories,
    ) -> anyhow::Result<String> {
        let model = favorite_categories::ActiveModel {
            id: Set(data.id.unwrap()),
            user_id: Set(data.user_id.unwrap()),
            name: Set(data.name),
            color: Set(data.color),
            ..Default::default()
        };

        let favorite_categories = model.update(self.pool.as_ref()).await?;
        Ok(favorite_categories.id.to_string())
    }

    // 删除收藏夹
    pub async fn delete_favorite_categories(&self, id: i64) -> anyhow::Result<()> {
        // 删除收藏夹下的房源
        user_favorites::Entity::delete_many()
            .filter(user_favorites::Column::CategoryId.eq(id))
            .exec(self.pool.as_ref())
            .await?;

        // 删除收藏夹
        favorite_categories::Entity::delete_many()
            .filter(favorite_categories::Column::Id.eq(id))
            .exec(self.pool.as_ref())
            .await?;

        Ok(())
    }

    // 查询用户收藏夹
    pub async fn find_favorite_categories(
        &self,
        user_id: String,
    ) -> anyhow::Result<Vec<favorite_categories::Model>> {
        favorite_categories::Entity::find()
            .filter(favorite_categories::Column::UserId.eq(user_id))
            .all(self.pool.as_ref())
            .await
            .map_err(|e| e.into())
    }

    // 房源添加收藏
    pub async fn add_user_favorites(&self, mut data: UserFavorites) -> anyhow::Result<String> {
        // 如果不存在收藏家就添加到默认收藏夹
        data.category_id = if data.category_id.is_none() {
            let category_id: Option<i64> = favorite_categories::Entity::find()
                .filter(favorite_categories::Column::Name.eq("我的收藏"))
                .select()
                .column(favorite_categories::Column::Id)
                .into_tuple()
                .one(self.pool.as_ref())
                .await?;

            if category_id.is_some() {
                category_id
            } else {
                // 创建默认收藏夹
                let category_id = self
                    .add_favorite_categories(FavoriteCategories {
                        id: None,
                        user_id: data.user_id.clone(),
                        name: "我的收藏".to_string(),
                        color: "#0073ffff".to_string(),
                    })
                    .await?;

                Some(category_id)
            }
        } else {
            data.category_id
        };

        let model = user_favorites::ActiveModel {
            user_id: Set(data.user_id.unwrap()),
            house_id: Set(data.house_id),
            category_id: Set(data.category_id),
            ..Default::default()
        };

        let favorite_categories = model.insert(self.pool.as_ref()).await?;
        Ok(favorite_categories.id.to_string())
    }

    // 移除收藏
    pub async fn cancel_user_favorites(&self, data: UserFavorites) -> anyhow::Result<()> {
        user_favorites::Entity::delete_many()
            .filter(
                user_favorites::Column::UserId
                    .eq(data.user_id.unwrap())
                    .and(user_favorites::Column::HouseId.eq(data.house_id)),
            )
            .exec(self.pool.as_ref())
            .await?;

        Ok(())
    }

    // 房源是否收藏
    pub async fn check_user_favorites(&self, data: UserFavorites) -> anyhow::Result<bool> {
        let count = user_favorites::Entity::find()
            .filter(user_favorites::Column::UserId.eq(data.user_id.unwrap()))
            .filter(user_favorites::Column::HouseId.eq(data.house_id))
            .count(self.pool.as_ref())
            .await?;

        Ok(count > 0)
    }

    // 房源更新收藏
    pub async fn update_user_favorites(&self, data: UserFavorites) -> anyhow::Result<String> {
        let model = user_favorites::ActiveModel {
            id: Set(data.id.unwrap()),
            user_id: Set(data.user_id.unwrap()),
            house_id: Set(data.house_id),
            category_id: Set(data.category_id),
            ..Default::default()
        };

        let favorite_categories = model.insert(self.pool.as_ref()).await?;
        Ok(favorite_categories.id.to_string())
    }

    // 查询用户收藏夹
    pub async fn find_user_favorite(
        &self,
        query: UserFavoriteQueryDto,
    ) -> anyhow::Result<Vec<HouseDataDto>> {
        let house_ids: Vec<String> = user_favorites::Entity::find()
            .filter(user_favorites::Column::UserId.eq(query.user_id.unwrap()))
            .filter(user_favorites::Column::CategoryId.eq(query.category_id))
            .select_only()
            .column(user_favorites::Column::HouseId)
            .into_tuple()
            .all(self.pool.as_ref())
            .await?;

        let condition = Condition::all()
            .add(house_query::Column::Id.is_in(house_ids))
            .add(house_query::Column::DeletedAt.is_null());

        let data = house_query::Entity::find()
            .join(
                JoinType::LeftJoin,
                house_query::Relation::CommunityQuery.def(),
            )
            .join(JoinType::LeftJoin, house_query::Relation::OwnerQuery.def())
            .filter(condition)
            .order_by_desc(house_query::Column::UpdatedAt)
            .select_also(community_query::Entity)
            .select_also(owner_query::Entity)
            .all(self.pool.as_ref())
            .await?;

        let data = data
            .into_iter()
            .map(|(house, community, owner)| HouseDataDto::new(house, community, owner))
            .collect::<Vec<HouseDataDto>>();

        Ok(data)
    }
}
