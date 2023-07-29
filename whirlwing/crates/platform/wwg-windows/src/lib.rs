use winit::dpi::{PhysicalSize, PhysicalPosition};

pub struct Window;

pub fn win_init() -> winit::window::Window {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Whirlwind Windows Window")
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_position(PhysicalPosition::new(200, 200))
        .with_resizable(false)
        .with_enabled_buttons(winit::window::WindowButtons::MINIMIZE | winit::window::WindowButtons::CLOSE)
        .build(&event_loop);

    wwg_log::wwg_trace!("Window is built!");
    
    window.unwrap()
}