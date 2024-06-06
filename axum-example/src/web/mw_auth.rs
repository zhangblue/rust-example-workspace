//! 自定义middleware

use axum::async_trait;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{error::Error, error::Result};

// 自定义中间件。用于登录状态检查；检查cookie中的内容是否正确
pub async fn mw_require_auth(
    ctx: Result<Ctx>, // 调用提取器并得到的参数
    req: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    let cc = ctx?;

    println!("从middleware中获取提取器得到的参数 : {}", cc.user_id());

    Ok(next.run(req).await)
}

// 自定义中间件。用于ctx的分析器，加速 from_request_parts 提取器在进行token验证时耗时，但可能被调用多次的情况
pub async fn mv_ctx_resolver(
    State(_mc): State<ModelController>, // 共享状态，里面可以放数据库连接。在此例子中没有使用
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:12} - mv_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // 验证cookie的合法性，并计算得到Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO token结构验证。验证通过后，将user_id 放入Ctx中
            println!("验证token的合法性, 验证逻辑可能需要链接数据库或某些缓存，这段逻辑需要运行很长时间....");
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    println!("current result_ctx is {:?}", result_ctx);

    // 如果出现了NoAuthTokenCookie以外的错误，则删除Cookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // 将result_ctx 放入 request 的扩展中。
    req.extensions_mut().insert(result_ctx); // 向请求中的扩展数据（extensions）中插入一个新的键值对。这个方法允许你在处理请求的过程中，将一些自定义的数据附加到请求上，以便后续处理中能够访问到这些数据。

    Ok(next.run(req).await)
}

// region: --- Ctx 提取器

// 自定义提取器，从 request 扩展空间中提取出Ctx。
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR",);

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

// endregion: --- Ctx 提取器

/// 解析token 格式为 `user-[user-id].[expiration].[signature]`
/// 返回信息 (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    // regex_captures 这个宏返回的内容 (正则能够匹配到的全部内容,第一个括号匹配到的内容user_id, 第二个括号匹配到的内容expiration, 第三个括号匹配到的内容signature)。如果匹配不到则返回错误信息
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    // 判断解析道的user_id是否是一个u64类型，如果不是则返回错误
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
