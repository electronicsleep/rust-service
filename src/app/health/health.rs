use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

impl HealthResponse {
    fn get() -> Self {
        HealthResponse {
            status: "Up".to_owned()
        }
    }
}

pub fn get() -> HealthResponse {
    HealthResponse::get()
}