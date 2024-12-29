use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod add_event;
mod get_events;
mod routes;

fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password))
}

#[actix_web::main]
/// Start Server
async fn main() -> std::io::Result<()> {
    let bind_address = "0.0.0.0:8081";
    println!("Server: http://{}", &bind_address);

    dotenv().ok();
    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set");
    let db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set");
    let db_port = db_port.parse().unwrap();

    let builder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    let pool = mysql::Pool::new(builder).unwrap();
    let shared_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(routes::svc_get_events)
            .service(routes::svc_add_event)
            .service(routes::index)
            .service(routes::home)
            .service(routes::about)
            .service(routes::item)
            .service(routes::itemid)
            .service(routes::echo)
            .service(routes::health)
    })
    .bind(bind_address)?
    .run()
    .await
}
