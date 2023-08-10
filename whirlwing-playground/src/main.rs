fn main() {
    let mut app = whirlwing::app::Application::new().unwrap();
    let _window = app.create_window("My Window", 50, 50, 1280, 720).unwrap();
    loop {}
}
