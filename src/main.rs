use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
extern crate r2d2;
extern crate r2d2_mysql;
use serde::Deserialize;
use std::env;

use r2d2::Pool;
use r2d2_mysql::mysql::{from_row, OptsBuilder, QueryResult};
use r2d2_mysql::MysqlConnectionManager;
use std::sync::Arc;

mod add_event;
mod get_events;

struct AppState {
    pool: Arc<Pool<MysqlConnectionManager>>,
}

fn setup_pool() -> Option<Arc<Pool<MysqlConnectionManager>>> {
    let datasource_user = env::var("datasource_user").unwrap_or("none".to_string());
    let datasource_db = env::var("datasource_db").unwrap_or("none".to_string());
    let datasource_password = env::var("datasource_password").unwrap_or("none".to_string());
    let mut o = OptsBuilder::new();
    o.db_name(Option::Some(datasource_db));
    o.user(Option::Some(datasource_user));
    o.pass(Option::Some(datasource_password));
    let manager = r2d2_mysql::MysqlConnectionManager::new(o);
    println!("INFO: setup_pool");
    let pool = Arc::new(r2d2::Pool::new(manager).unwrap());
    return Option::Some(pool);
}

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
    let events_list = serde_json::to_string(&events).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(events_list)
}

#[get("/event/{name}")]
/// Returns an event using main connection pool
async fn svc_get_event(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = path.into_inner();
    println!("INFO: Endpoint: /events/{}", name);

    //get connection from pool
    let pool = &data.pool;
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();

    let qr: QueryResult = conn
        .prep_exec(
            "SELECT event_id, service, event, event_type FROM events WHERE service = ?",
            ("infrasvc",),
        )
        .unwrap();

    let mut rec: Option<(String, String, String, String)> = None;

    for row in qr {
        rec = Some(from_row(row.unwrap()));
        break;
    }

    let unwrap_rec = rec.unwrap();
    format!("Record: {} ({})!", unwrap_rec.1, unwrap_rec.0);
    let json_data = format!("{{\"{}\": \"{}\"}}", unwrap_rec.1, unwrap_rec.2);
    println!("{}", json_data);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_data)
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
async fn svc_add_event(event: web::Json<Event>) -> impl Responder {
    println!("INFO: Endpoint: /add");
    let service = event.service.to_string();
    let event_name = event.event.to_string();
    let event_type = event.event_type.to_string();
    let datetime = event
        .datetime
        .as_ref()
        .unwrap_or(&"".to_owned())
        .to_string();
    let response = add_event::add_event(service, event_name, event_type, datetime);
    HttpResponse::Ok().body(format!("Event: {} ", response))
}

#[actix_web::main]
/// Start Server
async fn main() -> std::io::Result<()> {
    //prod
    //let bind_address = "0.0.0.0:8080";
    //dev
    let bind_address = "localhost:8080";

    let app_data = web::Data::new(AppState {
        pool: setup_pool().unwrap(),
    });

    println!("Server: http://{}", &bind_address);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .service(home)
            .service(about)
            .service(item)
            .service(itemid)
            .service(echo)
            .service(health)
            .service(svc_get_events)
            .service(svc_get_event)
            .service(svc_add_event)
    })
    .bind(bind_address)?
    .run()
    .await
}
