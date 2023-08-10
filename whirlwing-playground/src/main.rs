fn main() {
    let mut app = whirlwing::app::Application::new().unwrap();
    let window = app.create_window("My Window", 50, 50, 1280, 720).unwrap();
    app.make_current_window(&window);
    app.draw_unchecked(&window, 1.0, 1.0, 0.0, 1.0);
    loop {}
}
