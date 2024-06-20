// region:  --- Tocket Types

use serde::{Deserialize, Serialize};

// 用于从数据库查询后返回的
#[derive(Clone, Debug, Serialize)]
pub struct TicketVo {
    pub id: String,
    pub user_id: i32, // 创建这条数据的user_id
    pub title: String,
}

// 用于用户传入
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// endregion:  --- Tocket Types