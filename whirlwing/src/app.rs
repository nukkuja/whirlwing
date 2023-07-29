use wwg_events::{EventType, Event, EventCategory};
#[allow(unused_imports)]
use wwg_log::{wwg_err, wwg_info, wwg_trace, wwg_warn};
use wwg_window::Window;

pub struct Application {
    exit: bool,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        wwg_trace!("Whirlwing application is created!");
        Application {
            window: Window::new(),
            exit: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            wwg_info!("New application loop.");

            wwg_trace!("Processing events.");
            while let Some(event) = self.window.get_event() {
                if event.event_type() == EventType::ApplicationExit {
                    self.exit = true;
                }
            }

            wwg_trace!("Adding exit event to window.");
            self.window.add_event(Event::new(EventType::ApplicationExit, EventCategory::WindowEvent));

            if self.exit {
                break;
            }
        }
        wwg_info!("Application terminated.");
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
