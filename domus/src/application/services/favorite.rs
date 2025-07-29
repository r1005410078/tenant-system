use crate::application::listeners::house;
use crate::infrastructure::dto::house_data_dto::HouseDataDto;
use crate::infrastructure::entitiy::{
    community_query, favorite_categories, house_query, owner_query, user_favorites,
};
use sea_orm::{
    query, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JoinType, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait,
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
    pub async fn add_favorite_categories(
        &self,
        data: FavoriteCategories,
    ) -> anyhow::Result<String> {
        let model = favorite_categories::ActiveModel {
            user_id: Set(data.user_id.unwrap()),
            name: Set(data.name),
            color: Set(data.color),
            ..Default::default()
        };

        let favorite_categories = model.insert(self.pool.as_ref()).await?;
        Ok(favorite_categories.id.to_string())
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
    pub async fn add_user_favorites(&self, data: UserFavorites) -> anyhow::Result<String> {
        let model = user_favorites::ActiveModel {
            user_id: Set(data.user_id.unwrap()),
            house_id: Set(data.house_id),
            category_id: Set(data.category_id),
            ..Default::default()
        };

        let favorite_categories = model.insert(self.pool.as_ref()).await?;
        Ok(favorite_categories.id.to_string())
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

    // 移除收藏
    pub async fn delete_user_favorites(&self, id: i64) -> anyhow::Result<()> {
        user_favorites::Entity::delete_many()
            .filter(user_favorites::Column::Id.eq(id))
            .exec(self.pool.as_ref())
            .await?;

        Ok(())
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
