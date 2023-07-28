pub struct Window;

use wwg_events::*;

impl Window {
    pub fn send_event(&self) -> Event {
        wwg_events::Event::new(EventType::ApplicationExit, EventCategory::WindowEvent)
    }
}