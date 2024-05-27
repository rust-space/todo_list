use std::fmt::{Display, Formatter};

pub mod todo_srv;

// business error
#[derive(Debug)]
pub struct BizError {
    pub code: i32,
    pub msg: String,
}

impl Display for BizError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = format!("code: {},msg: {}", self.code, self.msg);
        std::fmt::Display::fmt(&s, f)
    }
}

impl std::error::Error for BizError {}

// error code
pub const SUCCESS_CODE: i32 = 0;
// 100xx todo module error
pub const TODO_QUERY_ERROR: i32 = 10000;
pub const TODO_CREATE_ERROR: i32 = 10001;
pub const TODO_UPDATE_ERROR: i32 = 10002;
pub const TODO_DELETE_ERROR: i32 = 10003;
