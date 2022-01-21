use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod app;

#[get("/")]
async fn index() -> impl Responder {
    println!("INFO: Endpoint: /");
    HttpResponse::Ok().body("Rust Service")
}

#[get("/health")]
async fn health() -> impl Responder {
    println!("INFO: Endpoint: /health");
    let status = "{\"status\": \"Up\"}";
    HttpResponse::Ok()
        .content_type("application/json")
        .body(status)
}

#[get("/events")]
async fn events(req_body: String) -> impl Responder {
    println!("INFO: Endpoint: /events");
    app::events::get_events();
    println!("DEBUG: req_body {}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct Event {
    service: String,
    event: String,
    event_type: String,
}

#[post("/add")]
async fn add_event(event: web::Json<Event>) -> impl Responder {
    let service = event.service.to_string();
    let event_name = event.event.to_string();
    let event_type = event.event_type.to_string();
    app::add_event::post_event(service, event_name, event_type);
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "0.0.0.0:8080";
    println!("Server: http://{}", &bind_address);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health)
            .service(events)
            .service(add_event)
    })
    .bind(bind_address)?
    .run()
    .await
}
