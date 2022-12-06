mod config;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use color_eyre::Result;
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::{Config, ConfigFromEnv};
use crate::models::Status;

const HOST_PATH: &'static str = "127.0.0.1:3333";

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

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
