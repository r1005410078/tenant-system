use std::sync::Arc;

use crate::infrastructure::entitiy::casbin_rules;
use casbin::Filter;
use casbin::{error::AdapterError, Adapter, Model, Result, TryIntoAdapter};
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;

pub struct SeaORMTryIntoAdapter {
    db: Arc<DatabaseConnection>,
}

impl SeaORMTryIntoAdapter {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        SeaORMTryIntoAdapter { db }
    }
}

#[async_trait::async_trait]
impl TryIntoAdapter for SeaORMTryIntoAdapter {
    async fn try_into_adapter(self) -> Result<Box<dyn Adapter>> {
        Ok(Box::new(SeaORMAdapter::new(self.db.clone())))
    }
}

struct SeaORMAdapter {
    db: Arc<DatabaseConnection>,
}

impl SeaORMAdapter {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        SeaORMAdapter { db }
    }
}

#[async_trait::async_trait]
impl Adapter for SeaORMAdapter {
    async fn load_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        // 实现加载策略的逻辑
        use sea_orm::EntityTrait;

        let rules = casbin_rules::Entity::find()
            .all(self.db.as_ref())
            .await
            .map_err(|e| AdapterError(Box::new(e)))?;

        for rule in rules {
            let ptype = rule.ptype.as_str();
            let rule_vec = vec![
                rule.v0.unwrap_or_default(),
                rule.v1.unwrap_or_default(),
                rule.v2.unwrap_or_default(),
            ];
            m.add_policy(ptype, ptype, rule_vec);
        }

        Ok(())
    }

    async fn load_filtered_policy<'a>(&mut self, _m: &mut dyn Model, _f: Filter<'a>) -> Result<()> {
        // 实现加载过滤后的策略的逻辑
        Ok(())
    }

    async fn save_policy(&mut self, _m: &mut dyn Model) -> Result<()> {
        println!("Saving policy...");
        // 实现保存策略的逻辑
        Ok(())
    }

    async fn clear_policy(&mut self) -> Result<()> {
        // 实现清除策略的逻辑
        Ok(())
    }

    fn is_filtered(&self) -> bool {
        false // 根据实际情况返回是否被过滤
    }

    async fn add_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        use sea_orm::ActiveModelTrait;

        // 实现添加策略的逻辑
        let ptype = ptype; // 假设所有规则的类型都是 "p"
        let new_rule = casbin_rules::ActiveModel {
            ptype: sea_orm::Set(ptype.to_string()),
            v0: sea_orm::Set(rule.get(0).cloned()),
            v1: sea_orm::Set(rule.get(1).cloned()),
            v2: sea_orm::Set(rule.get(2).cloned()),
            ..Default::default()
        };

        new_rule
            .insert(self.db.as_ref())
            .await
            .map_err(|e| AdapterError(Box::new(e)))?;

        Ok(true)
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        use sea_orm::ActiveModelTrait;

        // 实现添加多条策略的逻辑
        for rule in _rules {
            let ptype = "p"; // 假设所有规则的类型都是 "p"
            let new_rule = casbin_rules::ActiveModel {
                ptype: sea_orm::Set(ptype.to_string()),
                v0: sea_orm::Set(rule.get(0).cloned()),
                v1: sea_orm::Set(rule.get(1).cloned()),
                v2: sea_orm::Set(rule.get(2).cloned()),
                ..Default::default()
            };

            new_rule
                .insert(self.db.as_ref())
                .await
                .map_err(|e| AdapterError(Box::new(e)))?;
        }

        Ok(true)
    }

    async fn remove_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        // 实现移除策略的逻辑
        let ptype = ptype; // 假设所有规则的类型都是 "p"
        let new_rule = casbin_rules::ActiveModel {
            ptype: sea_orm::Set(ptype.to_string()),
            v0: sea_orm::Set(rule.get(0).cloned()),
            v1: sea_orm::Set(rule.get(1).cloned()),
            v2: sea_orm::Set(rule.get(2).cloned()),
            ..Default::default()
        };

        new_rule
            .delete(self.db.as_ref())
            .await
            .map_err(|e| AdapterError(Box::new(e)))?;

        // 如果删除成功，返回 true
        Ok(true)
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        // 实现移除多条策略的逻辑
        for rule in rules {
            let ptype = ptype; // 假设所有规则的类型都是 "p"
            let new_rule = casbin_rules::ActiveModel {
                ptype: sea_orm::Set(ptype.to_string()),
                v0: sea_orm::Set(rule.get(0).cloned()),
                v1: sea_orm::Set(rule.get(1).cloned()),
                v2: sea_orm::Set(rule.get(2).cloned()),
                ..Default::default()
            };

            new_rule
                .delete(self.db.as_ref())
                .await
                .map_err(|e| AdapterError(Box::new(e)))?;
        }

        Ok(true)
    }

    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _field_index: usize,
        _field_values: Vec<String>,
    ) -> Result<bool> {
        // 实现移除过滤后的策略的逻辑
        Ok(true)
    }
}
