//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_favorites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: String,
    pub house_id: String,
    pub category_id: Option<i64>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::favorite_categories::Entity",
        from = "Column::CategoryId",
        to = "super::favorite_categories::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    FavoriteCategories,

    #[sea_orm(
        belongs_to = "super::house_query::Entity",
        from = "Column::HouseId",
        to = "super::house_query::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    HouseQuery,
}

impl Related<super::favorite_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FavoriteCategories.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
