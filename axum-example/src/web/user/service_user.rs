use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::error;
use crate::web::common::{ApplicationStat};
use crate::web::user::model::{UserForCreate, UserVo};

use crate::entity;
use crate::entity::prelude;
use crate::error::Error;

pub async fn create_user(app_state: &ApplicationStat, user_fc: UserForCreate) -> error::Result<UserVo> {
    let create_at = chrono::Utc::now().naive_utc();

    // 模拟id递增，计算id
    let id = uuid::Uuid::new_v4().to_string();

    let user = entity::users::ActiveModel {
        id: Set(id),
        account: Set(user_fc.account),
        password: Set(user_fc.password),
        nick_name: Set(user_fc.nick_name),
        create_time: Set(create_at),
    };

    let insert_result = user.insert(app_state.db_conn.as_ref()).await;

    return match insert_result {
        Ok(data) => {
            let user_vo = UserVo {
                id: data.id,
                account: data.account,
                password: data.password,
                nick_name: data.nick_name,
                create_time: data.create_time.and_utc().timestamp(),
            };
            Ok(user_vo)
        }
        Err(error) => {
            Err(Error::DatabaseConnectionFail { msg: error.to_string() })
        }
    };
}

pub async fn list_user(app_state: &ApplicationStat) -> error::Result<Vec<UserVo>> {
    let vec_data = prelude::Users::find().all(app_state.db_conn.as_ref()).await;

    if let Err(err) = vec_data {
        return Err(Error::DatabaseConnectionFail { msg: err.to_string() });
    }

    let user_vo_vec: Vec<UserVo> = vec_data.unwrap().into_iter().map(|user| UserVo {
        id: user.id,
        account: user.account,
        password: user.password,
        nick_name: user.nick_name,
        create_time: user.create_time.and_utc().timestamp(),
    }).collect();

    Ok(user_vo_vec)
}

pub async fn delete_user(app_state: &ApplicationStat, id: &str) -> error::Result<UserVo> {
    let delete_model = get_user_one(app_state, id).await?;

    return match delete_model {
        None => {
            Err(Error::UserDeleteFailIdNotFound { id: id.to_string() })
        }
        Some(data) => {
            let _ = prelude::Users::delete_by_id(id).exec(app_state.db_conn.as_ref()).await;
            Ok(data)
        }
    };
}


pub async fn get_user_one(app_stat: &ApplicationStat, id: &str) -> error::Result<Option<UserVo>> {
    let res = prelude::Users::find_by_id(id).one(app_stat.db_conn.as_ref()).await;
    if let Err(err) = res {
        return Err(Error::DatabaseConnectionFail { msg: err.to_string() });
    }

    return match res.unwrap() {
        None => {
            Ok(None)
        }
        Some(data) => {
            let user_vo = UserVo {
                id: data.id,
                account: data.account,
                password: data.password,
                nick_name: data.nick_name,
                create_time: data.create_time.and_utc().timestamp(),
            };
            Ok(Some(user_vo))
        }
    };
}