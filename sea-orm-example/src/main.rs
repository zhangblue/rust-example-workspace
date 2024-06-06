use std::time::Duration;

use anyhow::{Ok, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

mod entities;
mod sql_inject;

#[tokio::main]
async fn main() {
    let conn = get_database_connection().await.unwrap();

    sql_inject::get_user_by_name("abc' or 1=1", &conn).await;
}

async fn get_database_connection() -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new("postgres://postgres:12345678@127.0.0.1:5432/rs_counter");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await?;

    Ok(db)
}
