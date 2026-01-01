use chrono;
use mysql::prelude::Queryable;
use mysql::*;

pub fn add_event(
    pool: &mysql::Pool,
    api_key: String,
    service: String,
    event: String,
    event_type: String,
    mut datetime: String,
) -> String {
    println!("INFO: add_event: {api_key} {service} {event} {event_type} {datetime}");
    println!("INFO: add_event: endpoint get conn");
    let mut conn: PooledConn = pool.get_conn().unwrap();

    if datetime == "" {
        println!("INFO: add_event: generate datetime now");
        let now = chrono::Utc::now();
        datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    let id = conn
        .exec_drop(
            "INSERT INTO events (event_id, service, event, event_type, datetime) VALUES (UUID(), ?, ?, ?, ?)",
            (service, event, event_type, datetime));

    println!("INFO: add_event: result {:?}", id);
    return "add_event ok\n".to_string();
}
