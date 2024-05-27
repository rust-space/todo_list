use self::todo::BaseResponse;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};

use super::service::SUCCESS_CODE;

pub mod not_found;
pub mod todo;

pub fn json_error(sc: StatusCode, code: i32, msg: String) -> (StatusCode, impl IntoResponse) {
    (
        sc,
        Json(BaseResponse {
            code,
            msg: msg.to_string(),
        }),
    )
}

pub fn json_succ(msg: String) -> impl IntoResponse {
    Json(BaseResponse {
        code: SUCCESS_CODE,
        msg: msg.to_string(),
    })
}
