#[allow(unused_imports)]
use whirlwing::{app, log, window};

fn main() {
    let mut app = app::Application::new(window::WindowDescriptor::default());
    app.run();

    let now = std::time::Instant::now();
    loop {
        if now.elapsed().as_secs() >= 5 {
            break;
        }
    }
}
