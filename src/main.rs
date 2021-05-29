use actix;

mod app;

fn main() {
    let sys = actix::System::new("rust_service");
    app::start();
    let _ = sys.run();
}
