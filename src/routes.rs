use actix_web::{HttpResponse, Responder, get, post, web};
use serde::Deserialize;

use crate::{add_event::add_event, get_events::get_events};

#[get("/")]
async fn index() -> HttpResponse {
    println!("INFO: Endpoint: /");
    HttpResponse::Ok().body("Rust Service\n")
}

#[get("/home")]
async fn home() -> impl Responder {
    println!("INFO: Endpoint: /home");
    let home = "Welcome to Actix Rust home!\n";
    HttpResponse::Ok().body(home)
}

#[get("/about")]
async fn about() -> impl Responder {
    println!("INFO: Endpoint: /about");
    let about = "Welcome to Actix Rust about page! \n\
        This App connects reads and inserts database records \n\
        into a MySQL database.\n";
    HttpResponse::Ok().body(about)
}

#[get("/health")]
/// Health check
async fn health() -> impl Responder {
    println!("INFO: Endpoint: /health");
    let status = "{\"status\": \"Up\"}";
    HttpResponse::Ok()
        .content_type("application/json")
        .body(status)
}

#[get("/item/{name}")]
/// Item accepts a string for item
async fn item(path: web::Path<String>) -> impl Responder {
    println!("INFO: Endpoint: /item/name");
    HttpResponse::Ok().body(format!("Item: {}", path.into_inner()))
}

#[get("/itemid/{id}")]
/// ItemId accepts a u32
async fn itemid(path: web::Path<u32>) -> impl Responder {
    println!("INFO: Endpoint: /itemid/id");
    HttpResponse::Ok().body(format!("ItemId: {}", path.into_inner()))
}

#[post("/echo")]
/// Will echo json body request sent
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/events")]
/// Display a list of events added to database
async fn svc_get_events(data: web::Data<mysql::Pool>) -> impl Responder {
    println!("INFO: Endpoint: /events");
    let events = get_events(&data);
    let events_list = serde_json::to_string(&events).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(events_list)
}

#[derive(Deserialize)]
struct Event {
    service: String,
    event: String,
    event_type: String,
    datetime: Option<String>,
}

#[post("/add")]
/// Add an event to the database
async fn svc_add_event(data: web::Data<mysql::Pool>, event: web::Json<Event>) -> impl Responder {
    println!("INFO: Endpoint: /add");

    let service = event.service.to_string();
    let event_name = event.event.to_string();
    let event_type = event.event_type.to_string();
    let datetime = event
        .datetime
        .as_ref()
        .unwrap_or(&"".to_owned())
        .to_string();
    let response = add_event(&data, service, event_name, event_type, datetime);
    HttpResponse::Ok().body(format!("Event: {} ", response))
}
