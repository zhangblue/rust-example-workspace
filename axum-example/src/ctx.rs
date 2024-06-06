//! 存储请求上下文的结构
#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64, // 用户id，从cookie解析出来的
}

// 构造器
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Ctx { user_id }
    }
}

// 参数访问
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
