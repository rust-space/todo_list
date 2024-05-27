use rbatis::{executor::Executor, impl_insert, impl_select_page, impl_update, py_sql, rbdc::db::ExecResult, Error};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

pub struct Todo {
    pub id: Option<i64>,
    pub description: Option<String>,
    pub completed: Option<u8>,
}

impl_select_page!(Todo{select_page() =>"`where deleted_at is null order by created_at desc`"});

impl_insert!(Todo {});

impl_update!(Todo {});

#[py_sql(
  "`update todo set deleted_at = current_timestamp where id = #{id}`"
)]
pub async fn logic_delete(rb: &dyn Executor, id: i64) -> Result<ExecResult, Error> {
  impled!()
}