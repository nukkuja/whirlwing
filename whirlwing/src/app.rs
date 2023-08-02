#[allow(unused_imports)]
use wwg_events::{Event, EventCategory, EventType};
#[allow(unused_imports)]
use wwg_log::{wwg_err, wwg_info, wwg_trace, wwg_warn};
use wwg_window::Window;

pub struct Application {
    exit: bool,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        wwg_info!("Whirlwing application is created!");
        Application {
            window: Window::new(),
            exit: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            wwg_trace!("New application loop.");
        }
        // wwg_info!("Application terminated.");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_app() {
        use super::*;
        Application::new().run();
    }
}
