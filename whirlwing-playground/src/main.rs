#[allow(unused_imports)]
use whirlwing::{app, log, window};

fn main() {
    let wd = window::WindowDescriptor::default().with_size(1280,720);
    let mut app = app::Application::new(wd);
    app.run();

    let now = std::time::Instant::now();
    loop {
        if now.elapsed().as_secs() >= 5 {
            break;
        }
    }
}
