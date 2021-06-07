use actix_web::{HttpRequest, Responder, HttpResponse};

mod events;

pub async fn get(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(events::get())
}