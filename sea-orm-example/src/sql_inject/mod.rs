use sea_orm::{ColumnTrait, DatabaseConnection, EntityOrSelect, QueryFilter, QueryTrait};

use crate::entities;

pub async fn get_user_by_name(name: &str, conn: &DatabaseConnection) {
    let select = entities::prelude::Users
        .select()
        .filter(entities::users::Column::SessionKey.eq(name));

    let sql = select.build(sea_orm::DatabaseBackend::Postgres);

    println!("sql = {}", sql);

    let result = select.all(conn).await;

    if let Ok(user_list) = result {
        println!("result size = {}", user_list.len());
    } else {
        println!("没找到");
    }
}
