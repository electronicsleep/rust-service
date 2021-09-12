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
        println!("INFO: add_event endpoint");
        println!("INFO NOW: {}", service);

        let datasource_conn_string =
            env::var("datasource_conn_string").unwrap_or("none".to_string());
        //println!("DEBUG: datasource_conn_string: {}", datasource_conn_string);

        let opts = Opts::from_url(&datasource_conn_string).unwrap();
        let pool = Pool::new(opts).unwrap();

        #[derive(Debug, PartialEq, Eq)]
        struct Events {
            event_id: i32,
            service: Option<String>,
            event: Option<String>,
            event_type: Option<String>,
            datetime: Option<String>,
        }

        println!("INFO: events endpoint get conn");

        let mut conn = pool.get_conn().unwrap();

        println!("INFO: Time Now {:?}", chrono::offset::Utc::now());
        let now = chrono::Utc::now();
        println!("{}", now.format("%Y-%m-%d %H:%M:%S").to_string());
        let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let query_start = "INSERT INTO events (service, event, event_type, datetime) VALUES(";
        let query = format!(
            "{}\"{}\", \"{}\", \"{}\", \"{}\")",
            query_start, service, event, event_type, datetime
        );
        println!("INFO: {:?}", query);

        let add_event = conn
            .query_map(query, |(event_id, service, event, event_type, datetime)| {
                Events {
                    event_id,
                    service,
                    event,
                    event_type,
                    datetime,
                }
            })
            .unwrap();

        println!("INFO: add_event endpoint");
        println!("INFO: {:?}", add_event);

        EventsResponse {
            status: "Ok".to_owned(),
        }
    }
}

pub fn post_event(service: String, event: String, event_type: String) -> EventsResponse {
    EventsResponse::post_event(service, event, event_type)
}
