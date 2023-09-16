#[allow(unused_imports)]
use whirlwing::{app, log, window};

fn main() {
    log::log_utils::set_engine_log_default(log::log_utils::Severity::Debug);
    let wd = window::WindowDescriptor::default().with_size(1280, 720);
    app::Application::new(wd).run();
}
