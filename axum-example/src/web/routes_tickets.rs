use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::ctx::Ctx;
use crate::error::Result;
use crate::model::{ModelController, Ticket, TicketForCreate};

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };

    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets)) // 相同的接口，调用post方法执行 create_ticket，调用get方式执行 list_tickets
        .route("/tickets/:id", delete(delete_ticket)) // 删除接口。调用 delete_ticket
        .with_state(app_state) // 这里设置的状态为 AppState 类型。
}

// region: --- REST Handlers
async fn create_ticket(
    State(mc): State<ModelController>, // 这里状态类型不必改为 AppState 类型，axum可以自动从 AppState 中得到 ModelController.
    ctx: Ctx,                          // 调用提取器并得到的参数
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    println!("从handler中获取到提取器中的参数 = {}", ctx.user_id());
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

// 这里状态类型不必改为 AppState 类型，axum可以自动从 AppState 中得到 ModelController.
async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>, // 这里状态类型不必改为 AppState 类型，axum可以自动从 AppState 中得到 ModelController.
    Path(id): Path<u64>,
    ctx: Ctx,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}

// endregion: --- REST Handlers
