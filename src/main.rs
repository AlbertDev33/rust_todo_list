use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use color_eyre::Result;

mod models;
use crate::models::Status;

const HOST_PATH: &'static str = "127.0.0.1:3333";

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status { status: "Ok".to_string() })
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(status))
    })
        .bind(HOST_PATH)?
        .run()
        .await?;

    Ok(())
}
