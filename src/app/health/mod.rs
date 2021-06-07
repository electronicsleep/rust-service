use actix_web::{HttpRequest, HttpResponse, Responder};

mod health;

pub async fn get(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(health::get())
}
