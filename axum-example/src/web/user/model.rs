// region:  --- user Types

use serde::{Deserialize, Serialize};

// 用于从数据库查询后返回的
#[derive(Clone, Debug, Serialize)]
pub struct UserVo {
    pub id: String,
    pub account: String,
    pub password: String,
    pub nick_name: String,
    pub create_time: i64,
}

// 用于用户传入
#[derive(Deserialize)]
pub struct UserForCreate {
    pub account: String,
    pub password: String,
    pub nick_name: String,
}

// endregion:  --- user Types