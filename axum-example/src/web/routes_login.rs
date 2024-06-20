use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{
    error::{Error, Result},
    web,
};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, Json(playload): Json<LoginPlayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: 实现真正的数据库权限登录
    if playload.username != "demo1" || playload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: 这里应该实现一个真正的授权token的 生成与识别
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // 创建 success body
    let body = Json(json!({
        "result":{
            "success":true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPlayLoad {
    username: String,
    pwd: String,
}
