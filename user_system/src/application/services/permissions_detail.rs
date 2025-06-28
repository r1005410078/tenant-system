use std::sync::Arc;

use crate::infrastructure::entitiy::{permissions_detail, prelude::PermissionsDetail};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{DbConn, EntityTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionsDetailDto {
    pub id: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,
    pub action: Option<String>,
    pub description: Option<String>,
}

pub struct PermissionsDetailService {
    pool: Arc<DbConn>,
}

impl PermissionsDetailService {
    pub fn new(pool: Arc<DbConn>) -> Self {
        Self { pool }
    }

    // 查询权限详情列表
    pub async fn list(&self) -> anyhow::Result<Vec<permissions_detail::Model>> {
        let model = PermissionsDetail::find().all(self.pool.as_ref()).await?;
        Ok(model)
    }

    // 或者更新权限详情
    pub async fn save(&self, data: Vec<PermissionsDetailDto>) -> anyhow::Result<()> {
        for d in data {
            let mut model = permissions_detail::ActiveModel {
                id: d.id.map_or(NotSet, Set),
                name: d.name.map_or(NotSet, Set),
                source: d.source.map_or(NotSet, Set),
                action: d.action.map_or(NotSet, Set),
                description: Set(d.description),
                ..Default::default()
            };

            if model.id.is_not_set() {
                model.id = Set(uuid::Uuid::new_v4().to_string());
                model.insert(self.pool.as_ref()).await?;
                continue;
            }

            model.update(self.pool.as_ref()).await?;
        }

        Ok(())
    }
}
