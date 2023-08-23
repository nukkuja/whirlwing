fn main() {
    let mut app =
        whirlwing::app::Application::new(whirlwing::window::WindowDescriptor::default());
    app.run();

    let now = std::time::Instant::now();
    loop {
        if now.elapsed().as_secs() >= 5 {
            break;
        }
    }
}
