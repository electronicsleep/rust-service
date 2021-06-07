use actix_web::{HttpRequest, HttpResponse, Responder};

mod events;

pub async fn get(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(events::get())
}
