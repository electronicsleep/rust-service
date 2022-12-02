use mysql::prelude::*;
use mysql::*;
use serde::Serialize;
use std::env;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Events {
    event_id: Option<String>,
    service: Option<String>,
    event: Option<String>,
    event_type: Option<String>,
    datetime: Option<String>,
}

pub fn get_events() -> Vec<Events> {
    println!("INFO: get_events");

    let datasource_conn_string = env::var("datasource_conn_string").unwrap_or("none".to_string());
    //println!("DEBUG: datasource_conn_string: {}", datasource_conn_string);

    let opts = Opts::from_url(&datasource_conn_string).unwrap();
    let pool = Pool::new(opts).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let selected_events = conn
        .query_map(
            "SELECT event_id, service, event, event_type, datetime FROM events ORDER BY datetime desc LIMIT 10",
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
    // println!("DEBUG: get_events: {:?}", selected_events);
    return selected_events;
}
