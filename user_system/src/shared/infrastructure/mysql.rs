use std::{env, sync::Arc};

use sea_orm::{Database, DatabaseConnection};

pub async fn create_mysql_pool() -> Arc<DatabaseConnection> {
    let db_url = env::var("CASBIN_DATABASE_URL")
        .unwrap_or("mysql://root:123456@192.168.1.10:3306/meida".to_string());

    let pool = Arc::new(Database::connect(&db_url).await.unwrap());
    pool
}
