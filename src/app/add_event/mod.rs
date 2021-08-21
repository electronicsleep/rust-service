use actix_web::{HttpResponse, Responder};

mod add_event;

pub fn post_event(service: String, event: String, event_type: String) -> impl Responder {
    HttpResponse::Ok().json(add_event::post_event(service, event, event_type))
}
