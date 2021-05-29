use actix_web::{HttpRequest, Responder, HttpResponse};

mod root;

pub async fn get(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(root::get())
}