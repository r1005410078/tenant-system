use casbin::{CoreApi, Enforcer};

use crate::shared::infrastructure::{
    casbin::sea_orm_adapter::SeaORMTryIntoAdapter, mysql::create_mysql_pool,
};

pub async fn init_casbin() -> Enforcer {
    let pool = create_mysql_pool().await;

    let sea_orm_try_into_adapter = SeaORMTryIntoAdapter::new(pool.clone());
    let mut e = Enforcer::new(
        "config/rbac_with_domains_model.conf",
        sea_orm_try_into_adapter,
    )
    .await
    .expect("Failed to create enforcer");

    e.enable_log(true);
    // 添加策略并自动保存
    e.enable_auto_save(true);

    e
}

#[cfg(test)]
mod tests {
    use casbin::CoreApi;
    use std::sync::Arc;

    use crate::shared::infrastructure::casbin::init_casbin::init_casbin;

    #[actix_web::test]
    async fn test_init_casbin() {
        let enforcer = Arc::new(init_casbin().await);
        let res = enforcer
            .enforce(("0475cffe-0833-4a38-a843-0134aa9f2cb9", "user", "update"))
            .unwrap();

        assert_eq!(res, true);
    }
}
