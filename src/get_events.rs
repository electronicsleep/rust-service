use mysql::prelude::Queryable;
use mysql::*;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Events {
    event_id: Option<String>,
    service: Option<String>,
    event: Option<String>,
    event_type: Option<String>,
    datetime: Option<String>,
}

pub fn get_events(pool: &mysql::Pool) -> Vec<Events> {
    println!("INFO: fn get_events");
    let mut conn: PooledConn = pool.get_conn().unwrap();

    let events = conn.query_map(
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

    // Print events
    println!("INFO: get_events: {:?}", events);
    return events;
}
