use actix_web::{HttpRequest, HttpResponse, Responder};

mod root;

pub async fn get(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(root::get())
}
