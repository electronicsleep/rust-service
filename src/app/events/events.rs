use mysql::prelude::*;
use mysql::*;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct EventsResponse {
    pub service: String,
    pub event: String,
    pub event_type: String,
}

impl EventsResponse {
    fn get() -> Self {
        println!("INFO: events endpoint");

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

        //let pool = Pool::new(datasource_conn_string).unwrap();

        println!("INFO: events endpoint get conn");

        let mut conn = pool.get_conn().unwrap();

        let selected_events = conn
            .query_map(
                "SELECT event_id, service, event, event_type, datetime from events",
                |(event_id, service, event, event_type, datetime)| Events {
                    event_id,
                    service,
                    event,
                    event_type,
                    datetime,
                },
            )
            .unwrap();

        println!("INFO: events endpoint, selected_events");
        println!("INFO: {:?}", selected_events);
        let events = selected_events.get(0).unwrap();
        println!("INFO: {:?}", events);

        EventsResponse {
            service: events.service.as_ref().unwrap().to_string(),
            event: events.event.as_ref().unwrap().to_string(),
            event_type: events.event_type.as_ref().unwrap().to_string(),
        }
    }
}

pub fn get() -> EventsResponse {
    EventsResponse::get()
}
