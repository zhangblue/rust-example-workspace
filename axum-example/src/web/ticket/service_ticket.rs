use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::{entity, error};
use crate::entity::prelude;
use crate::error::Error;
use crate::web::common::ApplicationStat;
use crate::web::ticket::model::{TicketForCreate, TicketVo};

pub async fn create_ticket(app_stat: &ApplicationStat, ctx: &Ctx, ticket_fc: TicketForCreate) -> error::Result<TicketVo> {
    // 模拟id递增，计算id
    let id = Uuid::new_v4().to_string();
    // 创建当前要入库的ticket
    let ticket = entity::tickets::ActiveModel {
        id: Set(id),
        user_id: Set(ctx.user_id()),
        title: Set(ticket_fc.title),
    };

    // 插入
    let insert_result = ticket.insert(app_stat.db_conn.as_ref()).await;

    return match insert_result {
        Ok(data) => {
            let ticket_vo = TicketVo {
                id: data.id,
                user_id: data.user_id,
                title: data.title,
            };
            Ok(ticket_vo)
        }
        Err(error) => {
            Err(Error::DatabaseConnectionFail { msg: error.to_string() })
        }
    };
}

pub async fn list_tickets(app_stat: &ApplicationStat, _ctx: &Ctx) -> error::Result<Vec<TicketVo>> {
    let vec_data = prelude::Tickets::find().all(app_stat.db_conn.as_ref()).await;

    if let Err(err) = vec_data {
        return Err(Error::DatabaseConnectionFail { msg: err.to_string() });
    }

    let ticket_vo_vec: Vec<TicketVo> = vec_data.unwrap().into_iter().map(|t| TicketVo {
        id: t.id,
        user_id: t.user_id,
        title: t.title,
    }).collect();

    Ok(ticket_vo_vec)
}

pub async fn delete_ticket(app_stat: &ApplicationStat, ctx: &Ctx, id: &str) -> error::Result<TicketVo> {
    let delete_model = get_ticket_one(app_stat, ctx, id).await?;

    return match delete_model {
        None => {
            Err(Error::TicketDeleteFailIdNotFound { id: id.to_string() })
        }
        Some(data) => {
            let _ = prelude::Tickets::delete_by_id(id).exec(app_stat.db_conn.as_ref()).await;
            Ok(data)
        }
    };
}

pub async fn get_ticket_one(app_stat: &ApplicationStat, _ctx: &Ctx, id: &str) -> error::Result<Option<TicketVo>> {
    let res = prelude::Tickets::find_by_id(id).one(app_stat.db_conn.as_ref()).await;

    if let Err(err) = res {
        return Err(Error::DatabaseConnectionFail { msg: err.to_string() });
    }

    return match res.unwrap() {
        None => {
            Ok(None)
        }
        Some(data) => {
            let ticket_vo = TicketVo {
                id: data.id,
                user_id: data.user_id,
                title: data.title,
            };
            Ok(Some(ticket_vo))
        }
    };
}

