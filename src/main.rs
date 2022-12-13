mod config;
mod models;
mod handlers;
mod db;

use actix_web::{web, App, HttpServer};
use color_eyre::Result;
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::{Config, ConfigFromEnv};
use crate::handlers::*;

const HOST_PATH: &'static str = "127.0.0.1:3333";

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    println!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{list_id}/items", web::get().to(get_items))
            .route("todos/{list_id}/items/{item_id}", web::put().to(check_item))
    })
    .bind(HOST_PATH)?
    .run()
    .await?;

    Ok(())
}
