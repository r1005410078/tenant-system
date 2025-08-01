//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "community_query")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub year_built: Option<DateTimeUtc>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub images: Option<Json>,
    #[sea_orm(column_type = "Double", nullable)]
    pub lat: f64,
    #[sea_orm(column_type = "Double", nullable)]
    pub lng: f64,
    pub typecode: String,
    pub district: Option<String>,
    pub adcode: Option<String>,
    pub property_management_company: Option<String>,
    pub remark: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
