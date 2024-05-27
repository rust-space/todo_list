use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        Json, Path, Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use rbatis::{Page, RBatis};
use serde::{Deserialize, Serialize};

use crate::biz::{handler::json_succ, model::todo::Todo, service::{todo_srv, SUCCESS_CODE}};

use super::json_error;

#[derive(Debug, Deserialize)]
pub struct TodosParams {
    page: u64,
    page_size: u64,
}

impl Default for TodosParams {
    fn default() -> Self {
        TodosParams {
            page: 1,
            page_size: 10,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateDto {
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDto {
    pub id: i64,
    pub description: String,
    pub completed: u8,
}

#[derive(Debug, Serialize)]
pub struct BaseResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct TodosResponse {
    pub code: i32,
    pub msg: String,
    pub data: Page<Todo>,
}

pub async fn todos(
    params: Result<Query<TodosParams>, QueryRejection>,
    State(rb): State<RBatis>,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    let p = params.unwrap_or_default();
    let data = todo_srv::query(rb, p.page, p.page_size)
        .await
        .map_err(|e| json_error(StatusCode::INTERNAL_SERVER_ERROR, e.code, e.msg))?;

    Ok(Json(TodosResponse {
        code: SUCCESS_CODE,
        msg: "success".to_string(),
        data,
    }))
}

pub async fn todo_create(
    State(rb): State<RBatis>,
    params: Result<Json<CreateDto>, JsonRejection>,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    let p = params.map_err(|e| {
        tracing::warn!("json parse error: {}", e);
        json_error(StatusCode::BAD_REQUEST, 400, "invalid params".to_string())
    })?;

    let msg = todo_srv::create(rb, p.0.description.as_ref())
        .await
        .map_err(|e| json_error(StatusCode::INTERNAL_SERVER_ERROR, e.code, e.msg))?;
    Ok(json_succ(msg))
}

pub async fn todo_update(
    State(rb): State<RBatis>,
    params: Result<Json<UpdateDto>, JsonRejection>,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    let p = params.map_err(|e| {
        tracing::warn!("json parse error: {}", e);
        json_error(StatusCode::BAD_REQUEST, 400, "invalid params".to_string())
    })?;

    let msg = todo_srv::update(rb, p.0)
        .await
        .map_err(|e| json_error(StatusCode::INTERNAL_SERVER_ERROR, e.code, e.msg))?;
    Ok(json_succ(msg))
}

pub async fn todo_delete(
    State(rb): State<RBatis>,
    params: Result<Path<i64>, PathRejection>,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    let p = params.map_err(|e| {
        tracing::warn!("path parse error: {}", e);
        json_error(StatusCode::BAD_REQUEST, 400, "invalid params".to_string())
    })?;
    let msg = todo_srv::delete(rb, p.0)
        .await
        .map_err(|e| json_error(StatusCode::INTERNAL_SERVER_ERROR, e.code, e.msg))?;
    Ok(json_succ(msg))
}
