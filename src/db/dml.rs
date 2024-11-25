use crate::{server::router::FormParams, settings::errors::MyError};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Row, Statement};

use crate::db::models::Test;

// retrieve test records list
pub async fn get_test_records(client: &Client) -> Result<Vec<Test>, MyError> {
    let _stmt: &str = include_str!("./sql/test/get_records.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let results: Vec<Test> = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Test::from_row_ref(row).unwrap())
        .collect::<Vec<Test>>();

    Ok(results)
}

// add test record
pub async fn add_test_record(client: &Client, test: FormParams) -> Result<i32, MyError> {
    let _stmt: &str = include_str!("./sql/test/add_record.sql");
    let stmt: Statement = client.prepare(&_stmt).await.unwrap();

    let rows: Vec<Row> = client.query(&stmt, &[&test.test_field]).await.unwrap();

    let idx: i32 = rows[0].get(0);

    Ok(idx)
}
