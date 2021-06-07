use actix_web::{web, App, HttpServer};

mod events;
mod health;
mod root;

pub fn start() {
    let bind_address = "0.0.0.0:8080";
    HttpServer::new(|| App::new().configure(routes))
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run();

    println!("Server: http://{}", &bind_address);
}

fn routes(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(root::get))
        .route("/events", web::get().to(events::get))
        .route("/health", web::get().to(health::get));
}
