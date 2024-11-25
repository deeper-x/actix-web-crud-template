use crate::db::{dml, models};
use crate::settings;
use actix_web::Result;
use actix_web::{web, Responder};
use askama::Template;
use deadpool_postgres::Client;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "form.html")]
struct Form;

#[derive(Serialize, Deserialize, Debug)]
pub struct FormParams {
    pub test_field: String,
}

// retrives ping records
pub async fn ping() -> impl Responder {
    "pong"
}

pub async fn form() -> Result<impl Responder> {
    let form = Form.render().expect("template should be valid");

    Ok(web::Html::new(form))
}

pub async fn save(
    params: web::Form<FormParams>,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(settings::errors::MyError::PoolError)?;
    println!("{:?}", params);
    let form_params = params.into_inner();

    let id: i32 = dml::add_test_record(&client, form_params).await?;

    Ok(id.to_string())
}

pub async fn get_all() -> Result<impl Responder> {
    Ok("todo")
}
