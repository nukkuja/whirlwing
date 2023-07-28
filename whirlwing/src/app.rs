use wwg_window::Window;
#[allow(unused_imports)]
use wwg_log::{ wwg_trace, wwg_info, wwg_warn, wwg_err };

pub struct Application {
    exit: bool,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        wwg_trace!("Whirlwing application is created!");
        Application { window: Window, exit: false }
    }

    pub fn run(&mut self) {
        loop { 
            wwg_info!("New application loop.");
            let event = self.window.send_event();
            wwg_info!("Received event from window: {:#?}", event);
            if event.event_type() == wwg_events::EventType::ApplicationExit {
                wwg_trace!("Received ApplicationExit event. Setting `exit` to true.");
                self.exit = true;
            }

            if self.exit { break; }
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