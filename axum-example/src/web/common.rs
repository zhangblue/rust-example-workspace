use std::sync::Arc;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use crate::connector::create_database_connection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub mc: ApplicationStat,
}

// region:  --- Model Controller

// 用于模拟数据库 会有多线程访问，所以需要使用arc，同时要加锁。
#[derive(Clone)]
pub struct ApplicationStat {
    pub db_conn: Arc<DatabaseConnection>,
}

impl ApplicationStat {
    pub async fn new() -> crate::error::Result<Self> {
        let db_connect = create_database_connection().await.expect("数据库链接失败!");

        Ok(Self {
            db_conn: Arc::new(db_connect)
        })
    }
}