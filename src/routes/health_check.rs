

use actix_web::{HttpResponse, Responder};
use serde::Deserialize;
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}