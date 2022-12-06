use actix_web::{HttpResponse, Responder};
use crate::models::Status;

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}