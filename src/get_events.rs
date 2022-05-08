use mysql::prelude::*;
use mysql::*;
use serde::Serialize;
use std::env;

#[derive(Serialize, Debug)]
pub struct Event {
    pub service: String,
    pub event: String,
    pub event_type: String,
    pub datetime: String,
}

pub fn get_events() -> Event {
    println!("INFO: get_events");

    let datasource_conn_string = env::var("datasource_conn_string").unwrap_or("none".to_string());
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

    let mut conn = pool.get_conn().unwrap();

    let selected_events = conn
        .query_map(
            "SELECT event_id, service, event, event_type, datetime FROM events",
            |(event_id, service, event, event_type, datetime)| Events {
                event_id,
                service,
                event,
                event_type,
                datetime,
            },
        )
        .unwrap();

    // Print all events
    println!("INFO: events: {:?}", selected_events);

    // Get first event
    //let events = selected_events.get(0).unwrap();

    // Get last event
    let last_event = selected_events.last().unwrap();

    // Print last events
    println!("INFO: {:?}", last_event);

    // Assign last event to struct for return
    let event = Event {
        service: last_event.service.as_ref().unwrap().to_string(),
        event: last_event.event.as_ref().unwrap().to_string(),
        event_type: last_event.event_type.as_ref().unwrap().to_string(),
        datetime: last_event.datetime.as_ref().unwrap().to_string(),
    };

    return event;
}
