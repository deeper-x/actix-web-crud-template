use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "test")]
pub struct Test {
    pub value: String,
    pub ts_created: Option<SystemTime>,
}