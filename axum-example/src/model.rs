//! 一个简单的模型层
//! 使用一个模拟的存储层

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    ctx::Ctx,
    error::{Error, Result},
};

// region:  --- Tocket Types

// 用于从数据库查询后返回的
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub user_id: u64, // 创建这条数据的user_id
    pub title: String,
}

// 用于用户传入
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// endregion:  --- Tocket Types

// region:  --- Model Controller

// 用于模拟数据库 会有多线程访问，所以需要使用arc，同时要加锁。
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD 实现
impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        // 模拟id递增，计算id
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        // 创建当前要入库的ticket
        let ticket = Ticket {
            id,
            user_id: ctx.user_id(),
            title: ticket_fc.title,
        };
        // 模拟放入数据库中
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets: Vec<Ticket> = store
            .iter()
            // .filter(|x| x.is_some())
            .filter_map(|t| t.clone())
            .collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion:  --- Model Controller
