use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod add_event;
mod get_events;

#[get("/")]
async fn index() -> HttpResponse {
    println!("INFO: Endpoint: /");
    HttpResponse::Ok().body("Rust Service")
}

#[get("/home")]
async fn home() -> impl Responder {
    println!("INFO: Endpoint: /home");
    let home = "Welcome to Actix Rust home!";
    HttpResponse::Ok().body(home)
}

#[get("/about")]
async fn about() -> impl Responder {
    println!("INFO: Endpoint: /about");
    let about = "Welcome to Actix Rust about page! \n\
        This App connects reads and inserts database records \n\
        into a MySQL database.";
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
    println!("INFO: Endpoint: /item");
    HttpResponse::Ok().body(format!("Item: {}", path.into_inner()))
}

#[get("/itemid/{id}")]
/// ItemId accepts a u32
async fn itemid(path: web::Path<u32>) -> impl Responder {
    println!("INFO: Endpoint: /itemid");
    HttpResponse::Ok().body(format!("ItemId: {}", path.into_inner()))
}

#[post("/echo")]
/// Will echo json body request sent
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/events")]
/// Returns a list of events added to database
async fn svc_get_events() -> impl Responder {
    println!("INFO: Endpoint: /events");
    let events = get_events::get_events();
    println!("RETURN: {:?}", events);
    HttpResponse::Ok().body(format!(
        "Event: {} {} {} {}",
        events.event, events.event_type, events.service, events.datetime
    ))
}

#[derive(Deserialize)]
struct Event {
    service: String,
    event: String,
    event_type: String,
}

#[post("/add")]
/// Add an event to the database
async fn svc_add_event(event: web::Json<Event>) -> impl Responder {
    println!("INFO: Endpoint: /add");
    let service = event.service.to_string();
    let event_name = event.event.to_string();
    let event_type = event.event_type.to_string();
    let response = add_event::add_event(service, event_name, event_type);

    //app::add_event::post_event(service, event_name, event_type);
    HttpResponse::Ok().body(format!("Event: {} ", response))
}

#[actix_web::main]
/// Start Server
async fn main() -> std::io::Result<()> {
    let bind_address = "0.0.0.0:8080";
    println!("Server: http://{}", &bind_address);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(home)
            .service(about)
            .service(item)
            .service(itemid)
            .service(echo)
            .service(health)
            .service(svc_get_events)
            .service(svc_add_event)
    })
    .bind(bind_address)?
    .run()
    .await
}
