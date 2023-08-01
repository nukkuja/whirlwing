pub struct Window {
    wnd_internal: wwg_windows::WindowInternal,
    event_queue: VecDeque<Event>,
}

use std::collections::VecDeque;

use wwg_events::*;

impl Window {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        use wwg_windows::window;
        Window {
            wnd_internal: window(),
            event_queue: VecDeque::new(),
        }
    }

    #[inline]
    pub fn process_messages(&self) {
        self.wnd_internal.process_messages();
    }

    pub fn add_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    pub fn get_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }
}
