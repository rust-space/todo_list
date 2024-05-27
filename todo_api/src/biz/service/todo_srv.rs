use rbatis::{Page, PageRequest, RBatis};
use serde_json::json;

use crate::biz::{
    handler::todo::UpdateDto,
    model::todo::{self, Todo}, service::{TODO_CREATE_ERROR, TODO_DELETE_ERROR, TODO_QUERY_ERROR, TODO_UPDATE_ERROR},
};

use super::BizError;

pub async fn query(
  rb: RBatis,
  page: u64,
  page_size: u64,
) -> Result<Page<Todo>, BizError> {
  let data = Todo::select_page(&rb, &PageRequest::new(page, page_size))
      .await
      .map_err(|e| {
          tracing::error!("query error: {}", e);
          BizError {
            code: TODO_QUERY_ERROR,
            msg: "quesy error".to_string(),
        }
      })?;
  tracing::debug!("select_page = {}", json!(data));

  Ok(data)
}

pub async fn create(rb: RBatis, description: &str) -> Result<String, BizError> {
    let table = Todo {
        id: None,
        description: Some(description.to_string()),
        completed: None,
    };
    let data = Todo::insert(&rb, &table).await.map_err(|e| {
        tracing::error!("insert error: {}", e);
        BizError {
            code: TODO_CREATE_ERROR,
            msg: "insert error".to_string(),
        }
    })?;
    tracing::debug!("insert = {}", json!(data));
    Ok(String::from("success"))
}

pub async fn update(rb: RBatis, p: UpdateDto) -> Result<String, BizError> {
    let table = Todo {
        id: Some(p.id),
        description: Some(p.description.clone()),
        completed: Some(p.completed),
    };
    let data = Todo::update_by_column(&rb, &table, "id")
        .await
        .map_err(|e| {
            tracing::error!("update error: {}", e);
            BizError {
                code: TODO_UPDATE_ERROR,
                msg: "update error".to_string(),
            }
        })?;
    tracing::debug!("update = {}", json!(data));

    Ok(String::from("success"))
}

pub async fn delete(rb: RBatis, id: i64) -> Result<String, BizError> {
    let data = todo::logic_delete(&rb, id).await.map_err(|e| {
        tracing::error!("delete error: {}", e);
        BizError {
            code: TODO_DELETE_ERROR,
            msg: "delete error".to_string(),
        }
    })?;
    tracing::debug!("delete = {}", json!(data));

    Ok(String::from("success"))
}
