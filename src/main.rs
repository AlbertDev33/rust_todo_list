mod config;
mod models;
mod handlers;

use actix_web::{web, App, HttpServer};
use color_eyre::Result;
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::{Config, ConfigFromEnv};
use crate::handlers::*;

const HOST_PATH: &'static str = "127.0.0.1:3333";

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    println!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(status))
    })
    .bind(HOST_PATH)?
    .run()
    .await?;

    Ok(())
}
