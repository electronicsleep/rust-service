use actix_web::{HttpResponse, Responder};

pub mod events;

pub fn get_events() -> impl Responder {
    HttpResponse::Ok().json(events::get_events())
}
