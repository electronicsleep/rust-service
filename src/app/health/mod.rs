use actix_web::{HttpRequest, Responder, HttpResponse};

mod health;

pub async fn get(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(health::get())
}