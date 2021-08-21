use actix_web::{HttpResponse, Responder};

pub mod events;

pub fn get_events() -> impl Responder {
    //println!("DEBUG get_events");
    HttpResponse::Ok().json(events::get_events())
}
