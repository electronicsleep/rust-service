use chrono;
use mysql::prelude::*;
use mysql::*;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct EventsResponse {
    pub status: String,
}

impl EventsResponse {
    fn post_event(service: String, event: String, event_type: String) -> Self {
        println!("INFO: post_event fn");

        let datasource_conn_string =
            env::var("datasource_conn_string").unwrap_or("none".to_string());
        //println!("DEBUG: datasource_conn_string: {}", datasource_conn_string);

        let opts = Opts::from_url(&datasource_conn_string).unwrap();
        let pool = Pool::new(opts).unwrap();

        #[derive(Debug, PartialEq, Eq)]
        struct Event {
            service: Option<String>,
            event: Option<String>,
            event_type: Option<String>,
            datetime: Option<String>,
        }

        println!("INFO: post_event endpoint get conn");

        let mut conn = pool.get_conn().unwrap();

        let now = chrono::Utc::now();
        //println!("DEBUG: {}", now.format("%Y-%m-%d %H:%M:%S").to_string());
        let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let id = conn.exec_drop(
            "INSERT INTO events (service, event, event_type, datetime) VALUES (:service, :event, :event_type, :datetime)",
            params! {
            "service" => service,
            "event" => event,
            "event_type" => event_type,
            "datetime" => datetime,
            },
        );

        println!("INFO: add_event endpoint");
        println!("INFO: {:?}", id);

        EventsResponse {
            status: "Ok".to_owned(),
        }
    }
}

pub fn post_event(service: String, event: String, event_type: String) -> EventsResponse {
    EventsResponse::post_event(service, event, event_type)
}
