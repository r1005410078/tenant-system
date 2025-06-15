use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::Arc};

pub async fn create_mysql_pool() -> Arc<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = Arc::new(Database::connect(&db_url).await.unwrap());
    pool
}
