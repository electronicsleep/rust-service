use serde::Serialize;

#[derive(Serialize)]
pub struct RootResponse {
    pub message: String,
}

impl RootResponse {
    fn get() -> Self {
        println!("INFO: root endpoint");
        RootResponse {
            message: "Hello Rust!".to_owned(),
        }
    }
}

pub fn get() -> RootResponse {
    RootResponse::get()
}
