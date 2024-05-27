extern crate rbatis;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use todo_api::biz::handler::{
    not_found::handler_404,
    todo::{todo_create, todo_delete, todo_update, todos},
};
use rbatis::RBatis;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let rb = RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver {},
        "mysql://root:123456@localhost:3305/demo",
    )
    .unwrap();

    // cors layer
    let cors = CorsLayer::new()
    .allow_headers(Any)
    .allow_origin(Any)
    .allow_methods(Any);

    let app = Router::new()
        .route("/todos", get(todos))
        .route("/todos", post(todo_create))
        .route("/todos", put(todo_update))
        .route("/todos/:id", delete(todo_delete))
        .layer(TraceLayer::new_for_http())
        .layer(cors.clone())
        .fallback(handler_404)
        .with_state(rb);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
